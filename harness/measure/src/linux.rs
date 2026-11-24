//! A run started, waited for and measured, by the system's own calls (Linux, 64 bits).

use crate::json::Value;
use std::ffi::OsString;
use std::fs::OpenOptions;
use std::os::unix::ffi::OsStringExt;
use std::os::unix::fs::OpenOptionsExt;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};
use std::time::{Duration, Instant};

/// How long, after a runner's first pause, its second is asked for over and over before it is
/// waited for asleep: longer than a refusal takes, and too short to cost a longer run a core.
const SPIN: Duration = Duration::from_micros(200);

const SIGKILL: i32 = 9;
const SIGCONT: i32 = 18;
const P_PID: i32 = 1;
const WNOHANG: i32 = 1;
const WSTOPPED: i32 = 2;
const WEXITED: i32 = 4;
const WNOWAIT: i32 = 0x0100_0000;
const CLD_STOPPED: i32 = 5;
const EINTR: i32 = 4;
const SC_CLK_TCK: i32 = 2;

/// `siginfo_t` as waitid fills it for a child: 128 bytes, the child's fields after three ints
/// and the padding that aligns the union they are in.
#[repr(C)]
struct SigInfo {
    signo: i32,
    errno: i32,
    code: i32,
    _pad: i32,
    pid: i32,
    uid: u32,
    status: i32,
    _rest: [u8; 100],
}

/// `struct rusage`: two `timeval`s, then fourteen longs, the first of them `ru_maxrss`.
#[repr(C)]
struct Rusage {
    utime: [i64; 2],
    stime: [i64; 2],
    maxrss: i64,
    _rest: [i64; 13],
}

/// `cpu_set_t`: a bit a core, 1024 of them.
type CpuSet = [u64; 16];

unsafe extern "C" {
    fn waitid(idtype: i32, id: u32, info: *mut SigInfo, options: i32) -> i32;
    fn wait4(pid: i32, status: *mut i32, options: i32, usage: *mut Rusage) -> i32;
    fn kill(pid: i32, signal: i32) -> i32;
    fn sysconf(name: i32) -> i64;
    fn sched_getaffinity(pid: i32, size: usize, set: *mut CpuSet) -> i32;
    fn sched_setaffinity(pid: i32, size: usize, set: *const CpuSet) -> i32;
}

/// How a run went, as the harness is told.
pub struct Ran {
    code: Option<i32>,
    seconds: f64,
    cpu: f64,
    peak_bytes: i64,
    own_seconds: Option<f64>,
    own_cpu: Option<f64>,
}

impl Ran {
    pub fn json(&self) -> String {
        fn or_null<T: std::fmt::Debug>(x: &Option<T>) -> String {
            x.as_ref().map_or("null".to_string(), |x| format!("{x:?}"))
        }
        format!(
            "{{\"code\": {}, \"seconds\": {:?}, \"cpu\": {:?}, \"peak_bytes\": {}, \"own_seconds\": {}, \"own_cpu\": {}}}",
            or_null(&self.code),
            self.seconds,
            self.cpu,
            self.peak_bytes,
            or_null(&self.own_seconds),
            or_null(&self.own_cpu)
        )
    }
}

fn os(bytes: &[u8]) -> OsString {
    OsString::from_vec(bytes.to_vec())
}

fn field<'a>(request: &'a Value, name: &str) -> Result<&'a Value, String> {
    request
        .get(name)
        .ok_or_else(|| format!("a request without `{name}`"))
}

fn text(request: &Value, name: &str) -> Result<OsString, String> {
    field(request, name)?
        .bytes()
        .map(os)
        .ok_or_else(|| format!("`{name}` is not a string"))
}

/// Bring this process's own peak down to what it holds now: it is where a runner's peak starts.
fn lower_peak() {
    let _ = std::fs::write("/proc/self/clear_refs", "5");
}

/// The user and system seconds process `pid` has spent so far, of every thread of it and of the
/// children it waited for, or None where /proc does not say. /proc's stat counts in clock ticks
/// (10 ms) but counts threads that have ended; the threads' schedstat counts in nanoseconds the
/// ones still running: the greater of the two is the nearer.
fn cpu_of(pid: u32) -> Option<f64> {
    let stat = std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    // the fields after the command's name, which may hold spaces: the state first
    let fields: Vec<&str> = stat.rsplit_once(')')?.1.split_whitespace().collect();
    let mut ticks = 0u64;
    for spent in fields.get(11..15)? {
        ticks += spent.parse::<u64>().ok()?;
    }
    // SAFETY: sysconf reads a constant of the system
    let per_second = unsafe { sysconf(SC_CLK_TCK) };
    if per_second <= 0 {
        return None;
    }
    let mut nanos = 0u64;
    for task in std::fs::read_dir(format!("/proc/{pid}/task"))
        .ok()?
        .flatten()
    {
        // a thread that ended as it was read counts nothing
        if let Ok(said) = std::fs::read_to_string(task.path().join("schedstat")) {
            nanos += said
                .split_whitespace()
                .next()
                .and_then(|n| n.parse::<u64>().ok())
                .unwrap_or(0);
        }
    }
    Some((ticks as f64 / per_second as f64).max(nanos as f64 / 1e9))
}

/// This process's cores, and it kept to `core` alone until this is dropped: a child takes its
/// affinity from the process that starts it, so it is pinned for the start only.
struct Pinned(CpuSet);

impl Pinned {
    fn to(core: usize) -> Result<Pinned, String> {
        let mut before: CpuSet = [0; 16];
        let mut only: CpuSet = [0; 16];
        *only
            .get_mut(core / 64)
            .ok_or_else(|| format!("no core {core}"))? |= 1 << (core % 64);
        // SAFETY: both sets are whole and of the size given
        let pinned = unsafe {
            sched_getaffinity(0, size_of::<CpuSet>(), &mut before) == 0
                && sched_setaffinity(0, size_of::<CpuSet>(), &only) == 0
        };
        if pinned {
            Ok(Pinned(before))
        } else {
            Err(format!("cannot run on core {core} alone"))
        }
    }
}

impl Drop for Pinned {
    fn drop(&mut self) {
        // SAFETY: the set is whole and of the size given
        unsafe { sched_setaffinity(0, size_of::<CpuSet>(), &self.0) };
    }
}

/// Wait for child `pid` to stop or exit, without reaping it: true if it stopped. Until
/// `spin_until` it is asked without sleeping, then waited for asleep.
fn stopped(pid: u32, spin_until: Option<Instant>) -> bool {
    let mut spinning = spin_until.is_some();
    loop {
        // SAFETY: zeroes are a siginfo, and waitid fills the one it is given
        let mut info: SigInfo = unsafe { std::mem::zeroed() };
        let options = WEXITED | WSTOPPED | WNOWAIT | if spinning { WNOHANG } else { 0 };
        // SAFETY: as above
        if unsafe { waitid(P_PID, pid, &mut info, options) } != 0 {
            if std::io::Error::last_os_error().raw_os_error() == Some(EINTR) {
                continue;
            }
            return false; // no such child: wait4 will say
        }
        if info.pid != 0 {
            return info.code == CLD_STOPPED;
        }
        // nothing yet (WNOHANG)
        if spin_until.is_some_and(|until| Instant::now() >= until) {
            spinning = false;
        } else {
            std::hint::spin_loop();
        }
    }
}

/// Start what `request` asks for, wait for it, and say how it ran; or why it could not be started.
pub fn run(request: &Value) -> Result<Ran, String> {
    let Value::Array(argv) = field(request, "argv")? else {
        return Err("`argv` is not a list".into());
    };
    let argv: Vec<OsString> = argv.iter().filter_map(Value::bytes).map(os).collect();
    let (program, arguments) = argv.split_first().ok_or("`argv` is empty")?;
    let Value::Object(env) = field(request, "env")? else {
        return Err("`env` is not an object".into());
    };
    let timeout = field(request, "timeout")?
        .number()
        .ok_or("`timeout` is not a number")?;
    let timeout = Duration::try_from_secs_f64(timeout).unwrap_or(Duration::MAX);
    let core = match request.get("core") {
        None | Some(Value::Null) => None,
        Some(core) => Some(
            core.number()
                .filter(|c| *c >= 0.0)
                .ok_or("`core` is not a core")? as usize,
        ),
    };
    let named = |what: std::io::Error| format!("{}: {what}", program.to_string_lossy());

    std::env::set_current_dir(text(request, "cwd")?).map_err(named)?;
    let file = |name: &str| -> Result<std::fs::File, String> {
        let path = text(request, name)?;
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(&path)
            .map_err(|e| format!("{}: {e}", path.to_string_lossy()))
    };
    let mut command = Command::new(program);
    command
        .args(arguments)
        .env_clear()
        .stdin(Stdio::null())
        .stdout(file("stdout")?)
        .stderr(file("stderr")?);
    for (name, value) in env {
        if let Some(value) = value.bytes() {
            command.env(name, os(value));
        }
    }

    let pinned = core.map(Pinned::to).transpose()?;
    lower_peak();
    let start = Instant::now();
    let child = command.spawn();
    drop(pinned);
    let pid = child.map_err(named)?.id();
    // the child is waited for here, by its pid, and not through `Child`: dropping that reaps nothing

    // the time limit: a thread that kills the runner unless told first that it has exited. It is
    // joined before the runner is reaped, and until then the pid is not another process's.
    let killed = Arc::new(AtomicBool::new(false));
    let (exited, exit) = mpsc::channel::<()>();
    let limit = std::thread::spawn({
        let killed = Arc::clone(&killed);
        move || {
            if exit.recv_timeout(timeout) == Err(mpsc::RecvTimeoutError::Timeout) {
                killed.store(true, Ordering::SeqCst);
                // SAFETY: kill only sends a signal, to a child not yet reaped
                unsafe { kill(pid as i32, SIGKILL) };
            }
        }
    });

    // the clock and the CPU time at each of the runner's pauses: its first two count
    let mut paused: Vec<(Instant, Option<f64>)> = Vec::new();
    let mut spin_until = None;
    while stopped(pid, spin_until.take()) {
        // a pause: at the first, what it has spent is read before the clock, and at the second
        // after it, so that reading /proc is not timed as the kernel's; and it goes on
        match paused.len() {
            0 => {
                let spent = cpu_of(pid);
                paused.push((Instant::now(), spent));
            }
            1 => {
                let now = Instant::now();
                paused.push((now, cpu_of(pid)));
            }
            _ => {}
        }
        // SAFETY: kill only sends a signal, to a child not yet reaped
        unsafe { kill(pid as i32, SIGCONT) };
        if paused.len() == 1 {
            spin_until = Some(Instant::now() + SPIN);
        }
    }
    let end = Instant::now();
    let _ = exited.send(());
    let _ = limit.join();

    let mut status = 0;
    // SAFETY: zeroes are an rusage, and wait4 fills the two it is given
    let mut usage: Rusage = unsafe { std::mem::zeroed() };
    // SAFETY: as above
    while unsafe { wait4(pid as i32, &mut status, 0, &mut usage) } < 0 {
        if std::io::Error::last_os_error().raw_os_error() != Some(EINTR) {
            return Err(format!(
                "{}: its exit could not be read",
                program.to_string_lossy()
            ));
        }
    }
    let signal = status & 0x7f;
    // ran out of time only if the limit's kill is what ended it, not an exit just before it
    let timed_out = killed.load(Ordering::SeqCst) && signal == SIGKILL;
    let cpu =
        (usage.utime[0] + usage.stime[0]) as f64 + (usage.utime[1] + usage.stime[1]) as f64 / 1e6;
    let (mut own_seconds, mut own_cpu) = (None, None);
    if let (Some(&(t0, c0)), false) = (paused.first(), timed_out) {
        // from the first pause to the second, or to its exit where it did not answer
        let (t1, c1) = paused.get(1).copied().unwrap_or((end, Some(cpu)));
        own_seconds = Some(t1.duration_since(t0).as_secs_f64());
        own_cpu = c0.zip(c1).map(|(c0, c1)| (c1 - c0).max(0.0));
    }
    Ok(Ran {
        // a signal's number, negative, as Python says an exit by one
        code: (!timed_out).then_some(if signal == 0 {
            (status >> 8) & 0xff
        } else {
            -signal
        }),
        seconds: end.duration_since(start).as_secs_f64(),
        cpu,
        // ru_maxrss is in kibibytes
        peak_bytes: usage.maxrss * 1024,
        own_seconds,
        own_cpu,
    })
}
