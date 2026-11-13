"""The harness's measurer: it starts each runner and waits for it, from a process that holds next to
nothing, so that the operating system's account of the runner's peak memory is the runner's own.

On Linux a process's peak resident set starts at the peak of the process that started it: a
runner started by the harness itself would have whatever the harness holds (every STL it has
read) counted as its own. This process holds only Python itself, and starts every runner for the
harness (harness/run.py starts it, `python3 -I -S harness/measure.py`).

It reads requests from its standard input, one line of JSON each:

    {"argv": [...], "cwd": "...", "env": {...}, "stdout": PATH, "stderr": PATH,
     "timeout": SECONDS, "core": N or null}

starts `argv` in `cwd` with `env`, its output to the two files and nothing on its input, and only
on core `core` if one is given; kills it if it runs past `timeout`; and answers one line of JSON:

    {"code": its exit code, or null if it ran out of time,
     "seconds": wall-clock, from starting it to its exit,
     "cpu": user and system seconds, of every thread of it and of the children it waited for,
     "peak_bytes": the most it held resident at once,
     "own_seconds": wall-clock, from its first pause to its second (or to its exit), or null,
     "own_cpu": user and system seconds over the same span, or null}

or {"error": "..."} if it could not be started. Standard library only; Linux and macOS.

A runner's pauses (SPEC.md, "A runner") are its warm start: under the harness a runner stops
itself (SIGSTOP) once it has loaded and found its case, and again once it has answered. Each time
it is stopped, this process reads the clock and the runner's CPU time and continues it (SIGCONT),
so `own_seconds` and `own_cpu` are the kernel's own, building and writing the solid: neither what
loading the runner costs nor what tearing it down does, and nothing subtracted. A runner that
never pauses has neither (null), and the board subtracts its start-up instead. `own_cpu` needs
/proc (Linux): elsewhere it is null.
"""
import json
import os
import signal
import sys
import time

# ru_maxrss is in kibibytes on Linux, in bytes on macOS
MAXRSS_BYTES = 1 if sys.platform == "darwin" else 1024
# the signals Python ignores, which a runner should not inherit ignored (as subprocess resets them)
RESET = [getattr(signal, s) for s in ("SIGPIPE", "SIGXFZ", "SIGXFSZ") if hasattr(signal, s)]


def lower_peak():
    """Bring this process's own peak down to what it holds now (Linux): it is where a runner's
    peak starts."""
    try:
        with open("/proc/self/clear_refs", "w") as f:
            f.write("5")
    except OSError:
        pass


def cpu_of(pid):
    """The user and system seconds process `pid` has spent so far, of every thread of it and of the
    children it waited for, or None where the system does not say (no /proc). /proc's stat counts
    in clock ticks (10 ms) but counts threads that have ended; the threads' schedstat counts in
    nanoseconds the ones still running: the greater of the two is the nearer."""
    try:
        with open(f"/proc/{pid}/stat") as f:
            # the fields after the command's name, which may hold spaces: the state first
            fields = f.read().rsplit(")", 1)[1].split()
        ticks = sum(int(x) for x in fields[11:15]) / os.sysconf("SC_CLK_TCK")
        nanos = 0
        for task in os.listdir(f"/proc/{pid}/task"):
            try:
                with open(f"/proc/{pid}/task/{task}/schedstat") as f:
                    nanos += int(f.read().split()[0])
            except (OSError, ValueError, IndexError):
                pass  # a thread that ended as it was read
        return max(ticks, nanos / 1e9)
    except (OSError, ValueError, IndexError):
        return None


def run(req):
    os.chdir(req["cwd"])
    flags = os.O_WRONLY | os.O_CREAT | os.O_TRUNC
    out = os.open(req["stdout"], flags, 0o600)
    err = os.open(req["stderr"], flags, 0o600)
    null = os.open(os.devnull, os.O_RDONLY)
    actions = [(os.POSIX_SPAWN_DUP2, null, 0), (os.POSIX_SPAWN_DUP2, out, 1),
               (os.POSIX_SPAWN_DUP2, err, 2)]
    core = req.get("core")
    before = os.sched_getaffinity(0) if core is not None else None
    try:
        # a child takes its affinity from the process that starts it: pinned for the start only
        if core is not None:
            os.sched_setaffinity(0, {core})
        lower_peak()
        start = time.monotonic()
        pid = os.posix_spawnp(req["argv"][0], req["argv"], req["env"], file_actions=actions,
                              setsigdef=RESET)
    except OSError as e:
        return {"error": f"{req['argv'][0]}: {e.strerror or e}"}
    finally:
        if before is not None:
            os.sched_setaffinity(0, before)
        for fd in (out, err, null):
            os.close(fd)

    killed, reaped = [], None

    def ran_out(signum, frame):
        killed.append(True)
        try:
            os.kill(pid, signal.SIGKILL)
        except ProcessLookupError:
            pass

    signal.signal(signal.SIGALRM, ran_out)
    signal.setitimer(signal.ITIMER_REAL, req["timeout"])
    # the clock and the CPU time at each of the runner's pauses: its first two count
    paused = []
    try:
        # wait for it to exit without reaping it: until it is reaped its pid is not reused, so a
        # timer that fires late kills only the exited process, which is harmless (no waitid on
        # macOS before Python 3.13: there wait4 reaps it at once)
        while True:
            if hasattr(os, "waitid"):
                info = os.waitid(os.P_PID, pid, os.WEXITED | os.WSTOPPED | os.WNOWAIT)
                stopped = info.si_code == os.CLD_STOPPED
            else:
                reaped = os.wait4(pid, os.WUNTRACED)
                stopped = os.WIFSTOPPED(reaped[1])
                if stopped:
                    reaped = None
            if not stopped:
                break
            # a pause: at the first, what it has spent is read before the clock, and at the
            # second after it, so that reading /proc is not timed as the kernel's; and it goes on
            if not paused:
                spent = cpu_of(pid)
                paused.append((time.monotonic(), spent))
            elif len(paused) == 1:
                now = time.monotonic()
                paused.append((now, cpu_of(pid)))
            try:
                os.kill(pid, signal.SIGCONT)
            except ProcessLookupError:
                pass
        seconds = time.monotonic() - start
    finally:
        signal.setitimer(signal.ITIMER_REAL, 0)
        signal.signal(signal.SIGALRM, signal.SIG_IGN)
    _, status, usage = reaped or os.wait4(pid, 0)
    # ran out of time only if the timer's kill is what ended it, not an exit just before it
    timed_out = (killed and os.WIFSIGNALED(status)
                 and os.WTERMSIG(status) == signal.SIGKILL)
    cpu = usage.ru_utime + usage.ru_stime
    own_seconds = own_cpu = None
    if paused and not timed_out:
        # from the first pause to the second, or to its exit where it did not answer
        (t0, c0), (t1, c1) = paused[0], (paused[1] if len(paused) > 1 else (start + seconds, cpu))
        own_seconds = t1 - t0
        own_cpu = None if c0 is None or c1 is None else max(0.0, c1 - c0)
    return {"code": None if timed_out else os.waitstatus_to_exitcode(status),
            "seconds": seconds, "cpu": cpu,
            "peak_bytes": usage.ru_maxrss * MAXRSS_BYTES,
            "own_seconds": own_seconds, "own_cpu": own_cpu}


def main():
    for line in sys.stdin:
        if line.strip():
            sys.stdout.write(json.dumps(run(json.loads(line))) + "\n")
            sys.stdout.flush()


if __name__ == "__main__":
    main()
