//! The harness's measurer: it starts each runner and waits for it, from a process that holds next
//! to nothing, so that the operating system's account of the runner's peak memory is the runner's
//! own, and it times the runner between its pauses as closely as a process can.
//!
//! `harness/run.py` builds and starts it, once for all its runs. It is for Linux: elsewhere a
//! run is timed whole by the harness itself and nothing else of it is measured. It is Rust
//! because the time between a runner's two pauses is as short as tens of microseconds, and an
//! interpreter between the runner's stopping and the clock's being read is a good part of that.
//!
//! It reads requests from its standard input, one line of JSON each:
//!
//! ```text
//! {"argv": [...], "cwd": "...", "env": {...}, "stdout": PATH, "stderr": PATH,
//!  "timeout": SECONDS, "core": N or null}
//! ```
//!
//! starts `argv` in `cwd` with `env`, its output to the two files and nothing on its input, and
//! only on core `core` if one is given; kills it if it runs past `timeout`; and answers one line
//! of JSON:
//!
//! ```text
//! {"code": its exit code, or null if it ran out of time,
//!  "seconds": wall-clock, from starting it to its exit,
//!  "cpu": user and system seconds, of every thread of it and of the children it waited for,
//!  "peak_bytes": the most it held resident at once,
//!  "own_seconds": wall-clock, from its first pause to its second (or to its exit), or null,
//!  "own_cpu": user and system seconds over the same span (from /proc), or null}
//! ```
//!
//! or `{"error": "..."}` if it could not be started.
//!
//! A runner's pauses (SPEC.md, "A runner") are its warm start: under the harness a runner stops
//! itself (SIGSTOP) once it has loaded and found its case, and again once it has answered. Each
//! time it is stopped, this process reads the clock and the runner's CPU time and continues it
//! (SIGCONT). After the first pause it does not sleep until the runner stops again, but asks over
//! and over for a short while (`SPIN`): a refusal is answered in microseconds, and being woken
//! would cost as many. A run that goes on longer is waited for asleep, leaving its cores to it.

mod json;
#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
fn main() {
    use std::io::{BufRead, Write};
    let stdout = std::io::stdout();
    for line in std::io::stdin().lock().lines() {
        let Ok(line) = line else { break };
        if line.trim().is_empty() {
            continue;
        }
        let answer = match json::parse(&line).and_then(|request| linux::run(&request)) {
            Ok(ran) => ran.json(),
            Err(why) => format!("{{\"error\": {}}}", json::quoted(&why)),
        };
        let mut out = stdout.lock();
        if writeln!(out, "{answer}")
            .and_then(|()| out.flush())
            .is_err()
        {
            break; // the harness is gone
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("harness/measure is for Linux: elsewhere the harness keeps only a run's time");
    std::process::exit(2);
}
