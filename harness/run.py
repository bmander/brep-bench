#!/usr/bin/env python3
"""Run every case against every kernel and grade the results.

    python3 harness/run.py                     # every kernel in kernels.json, every case
    python3 harness/run.py --kernel arris --case boolean/
    python3 harness/run.py --timeout 120
    python3 harness/run.py --plan plan.json    # what harness/changed.py says changed

A kernel's runner is asked for each case it has an implementation of (SPEC.md); a case it has none
of is missing. Writes results/<kernel>.json after a run of the whole suite (not of a part,
`--case`), or folds a plan's runs into the last one (`--plan`), and keeps each result's triangles
as results/meshes/<kernel>/<case>.stl for the drawings and the viewer.

What is measured is a kernel's shortest path to a boundary representation, and no more: each
case is run twice. The **timed run** gives the runner `-` in place of OUT.stl, so that it builds
the solid and, for a kernel whose runner writes STEP (`"step": true` in kernels.json), writes it
as STEP, and meshes and measures nothing; the **graded run**, untimed, gives it OUT.stl, and its
reply and triangles are graded. The timed run's wall-clock time, the CPU time it spent (user and
system, over every thread) and its peak memory (the largest resident set) are kept, from the
operating system's account of the runner's process (wait4), which harness/measure.py starts and
waits for; nothing measured is graded. A timed run that runs out of time or refuses is the
case's outcome, and the graded run is not made. So is what starting the runner costs, alone: it
is given a case id no case has, `harness/start`, so that it starts and builds nothing, a few times
over (`startup`). With `--one-core` a case the kernel built is given the timed run a second time
on one core only, for what its other cores bought it; that run is timed and nothing else. What
the STEP file of the timed run says the solid is made of is kept beside its result
(harness/step.py), or that its kernel's writer refused, or that the run failed; like the runner's
own `faces`, it is not graded. Times are kept to the microsecond: a kernel's own time, a run's
less its runner's start-up, can be under a millisecond. Standard library only (where there is no
wait4, only the time is kept; pinning to one core needs Linux).
"""
import argparse
import functools
import glob
import json
import os
import platform
import re
import shutil
import subprocess
import sys
import tempfile
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import step  # noqa: E402
import stl  # noqa: E402
from suite import ROOT, cases, implements, kernels, source  # noqa: E402

# How near a measure must be to the expected value, relative to it.
EXACT = 1e-6
# How near the volume of the runner's own triangles must be: a mesh is within its sag of the
# solid, so this only catches a mesh of some other solid.
MESHED = 2e-2
# How near the measures must be for a kernel with no exact measures of its own, which the harness
# then takes from the runner's triangles (`"measured": "mesh"`, SPEC.md).
FROM_MESH = 1e-3


def near(got, want, rel):
    return got is not None and abs(got - want) <= rel * max(abs(want), 1e-12)


def grade(case, reply, mesh):
    """The outcome of one run: pass (built for an unsettled case), refused, wrong (with why) or
    crashed."""
    status = reply.get("status")
    if status == "refused":
        return "refused", reply.get("reason", "")
    if status != "ok":
        return "crashed", f"status {status!r}"
    why = []
    exp = case["expect"]
    if exp is None:
        # unsettled: no answer to grade against, only whether the solid is sound
        if mesh is None:
            return "wrong", "no triangles written"
        if not mesh["closed"]:
            return "wrong", f"triangles not closed ({mesh['unmatched_edges']} edges unmatched)"
        return "built", ""
    from_mesh = reply.get("measured") == "mesh"
    for key in ("volume", "area"):
        got, want = reply.get(key), exp[key]["value"]
        if from_mesh:
            got = mesh and mesh[key]
        if not near(got, want, FROM_MESH if from_mesh else EXACT):
            why.append(f"{key} {got} against {want:.12g}")
    if mesh is None:
        why.append("no triangles written")
    else:
        if not mesh["closed"]:
            why.append(f"triangles not closed ({mesh['unmatched_edges']} edges unmatched)")
        if not near(mesh["volume"], exp["volume"]["value"], MESHED):
            why.append(f"triangles' volume {mesh['volume']:.6g}")
    return ("pass", "") if not why else ("wrong", "; ".join(why))


def why_crashed(stderr, code):
    """What a crashed runner said last: a Rust panic's place and message, else its last line."""
    lines = [l for l in stderr.strip().splitlines() if l.strip() and not l.startswith("note:")]
    for i, line in enumerate(lines):
        if "panicked at" in line:
            # the place in the kernel's source, not where cargo keeps it
            said = re.sub(r"\S*/\.cargo/(git/checkouts|registry/src)/[^/]+/[^/]+/", "",
                          " ".join(lines[i:i + 2]))
            return re.sub(r"^thread '[^']*'( \(\d+\))? ", "", said)[:300]
    return (lines[-1] if lines else f"exit {code}")[:300]


class Ran:
    """How a process ran: its exit `code` (None if it ran out of time), what it wrote, and what it
    cost: wall-clock `seconds`, `cpu` seconds (user and system, of every thread of it and of the
    children it waited for) and `peak_mb`, the most memory it held resident at once (megabytes,
    1e6 bytes). `cpu` and `peak_mb` are None where the system does not say (no wait4)."""

    def __init__(self, code, stdout, stderr, seconds, cpu=None, peak_mb=None):
        self.code, self.stdout, self.stderr = code, stdout, stderr
        self.seconds, self.cpu, self.peak_mb = seconds, cpu, peak_mb


# where it can, a run is started and waited for by harness/measure.py, a process that holds next
# to nothing: on Linux a process's peak memory starts at its starter's, which would count the
# harness's own (every STL it has read) against the runner
MEASURE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "measure.py")
MEASURED = all(hasattr(os, f) for f in ("posix_spawnp", "wait4"))
measurer = None


def measured(request):
    """What harness/measure.py answers of `request`, started the first time it is wanted."""
    global measurer
    if measurer is None or measurer.poll() is not None:
        measurer = subprocess.Popen([sys.executable, "-I", "-S", MEASURE], stdin=subprocess.PIPE,
                                    stdout=subprocess.PIPE, text=True, cwd=ROOT)
    measurer.stdin.write(json.dumps(request) + "\n")
    measurer.stdin.flush()
    line = measurer.stdout.readline()
    if not line:
        raise RuntimeError("harness/measure.py stopped")
    answer = json.loads(line)
    if "error" in answer:
        raise OSError(answer["error"])
    return answer


def execute(argv, timeout, core=None):
    """Run `argv` until it exits or `timeout` seconds pass, and say how it ran (`Ran`). With `core`,
    the process and every thread it starts may run on that core only."""
    env = dict(os.environ, RUST_BACKTRACE="0")
    with tempfile.TemporaryDirectory() as tmp:
        out, err = os.path.join(tmp, "stdout"), os.path.join(tmp, "stderr")
        if MEASURED:
            a = measured({"argv": argv, "cwd": ROOT, "env": env, "stdout": out, "stderr": err,
                          "timeout": timeout, "core": core})
            code, seconds, cpu, peak = a["code"], a["seconds"], a["cpu"], a["peak_bytes"] / 1e6
        else:
            # no wait4 (Windows): the time alone
            with open(out, "wb") as o, open(err, "wb") as e:
                start = time.monotonic()
                p = subprocess.Popen(argv, stdout=o, stderr=e, stdin=subprocess.DEVNULL,
                                     cwd=ROOT, env=env)
                try:
                    code = p.wait(timeout)
                except subprocess.TimeoutExpired:
                    p.kill()
                    p.wait()
                    code = None
                seconds = time.monotonic() - start
            cpu = peak = None
        with open(out, encoding="utf-8", errors="replace") as o, \
                open(err, encoding="utf-8", errors="replace") as e:
            return Ran(code, o.read(), e.read(), seconds, cpu, peak)


# what the timed run gives the runner for OUT.stl: no triangles to write, nothing to measure
# (SPEC.md, "A runner")
NO_MESH = "-"
# the outcomes of a run that built a solid: its STEP is read, and it is run again on one core
BUILT = ("pass", "built", "wrong")


def said(ran):
    """A runner's reply, the last line of its standard output as JSON, or None."""
    try:
        reply = json.loads(ran.stdout.strip().splitlines()[-1])
    except (ValueError, IndexError):
        return None
    return reply if isinstance(reply, dict) else None


def timed_argv(command, cid, out_step):
    """The timed run's command line: the case, `-` for OUT.stl, and OUT.step where the kernel's
    runner writes STEP (`out_step`, else None)."""
    return command + [cid, NO_MESH] + ([out_step] if out_step else [])


def graded(command, case, timeout, keep):
    """The graded run of one case, untimed: the runner given OUT.stl, its reply and triangles
    graded (outcome, why, reply, mesh). The triangles are kept as `keep`."""
    with tempfile.TemporaryDirectory() as tmp:
        out_stl = os.path.join(tmp, "result.stl")
        ran = execute(command + [case["id"], out_stl], timeout)
        if ran.code is None:
            return "timeout", f"over {timeout} s (meshing it)", None, None
        if ran.code != 0:
            return "crashed", why_crashed(ran.stderr, ran.code), None, None
        reply = said(ran)
        if reply is None:
            return "crashed", "no JSON reply", None, None
        mesh = None
        if reply.get("status") == "ok" and os.path.exists(out_stl):
            try:
                mesh = stl.check(stl.read(out_stl))
            except (ValueError, OSError) as e:
                return "wrong", f"unreadable STL: {e}", reply, None
            # kept for the drawings and the viewer (results/meshes, not committed)
            os.makedirs(os.path.dirname(keep), exist_ok=True)
            shutil.copyfile(out_stl, keep)
        return (*grade(case, reply, mesh), reply, mesh)


# the largest STEP file kept for the viewer, in bytes: a faceted solid's runs to tens of
# megabytes, which the results' cache and the site would carry for every run; its make-up is kept
# whatever its size
STEP_KEPT = 5_000_000


def made_of(ran, reply, out_step, keep):
    """The make-up of the solid in STEP, as the timed run wrote it to `out_step` (SPEC.md, "STEP").
    The file is kept as `keep`, for the viewer, if it is no larger than STEP_KEPT. Where there is
    no file to read: {"refused": why} where the runner said why its kernel's writer declined
    (`step_error`), else {"error": why}, a failure: the runner crashed, wrote nothing and said
    nothing, or wrote what could not be read."""
    if ran.code != 0:
        return {"error": why_crashed(ran.stderr, ran.code)}
    if not os.path.exists(out_step):
        if reply and reply.get("step_error"):
            return {"refused": str(reply["step_error"])}
        return {"error": "no STEP written, and no step_error saying why"}
    try:
        makeup = step.read(out_step)
    except (ValueError, OSError) as e:
        return {"error": f"unreadable STEP: {e}"}
    if os.path.getsize(out_step) <= STEP_KEPT:
        os.makedirs(os.path.dirname(keep), exist_ok=True)
        shutil.copyfile(out_step, keep)
    return makeup


def run_one(command, case, timeout, keep, keep_step, writes_step):
    """Run and grade one case (SPEC.md, "A runner"): first the timed run, the runner given `-` for
    OUT.stl, building the solid and, where `writes_step`, writing it as STEP; then, unless that
    ran out of time or refused, the graded run, untimed, with OUT.stl. (outcome, why, reply, mesh,
    how the timed run ran, its STEP's make-up or None)."""
    with tempfile.TemporaryDirectory() as tmp:
        out_step = os.path.join(tmp, "result.step") if writes_step else None
        ran = execute(timed_argv(command, case["id"], out_step), timeout)
        if ran.code is None:
            return "timeout", f"over {timeout} s", None, None, ran, None
        reply = said(ran) if ran.code == 0 else None
        if reply and reply.get("status") == "refused":
            return "refused", reply.get("reason", ""), reply, None, ran, None
        outcome, why, graded_reply, mesh = graded(command, case, timeout, keep)
        made = (made_of(ran, reply, out_step, keep_step)
                if writes_step and outcome in BUILT else None)
        return outcome, why, graded_reply, mesh, ran, made


# the decimal places a time in seconds is kept to: microseconds
TIME_PLACES = 6


def one_core_seconds(command, case, timeout, core, writes_step):
    """The wall-clock time of the case's timed run made again on `core` alone, or None if it ran
    out of time or failed. Nothing of the run but its time is kept."""
    with tempfile.TemporaryDirectory() as tmp:
        out_step = os.path.join(tmp, "result.step") if writes_step else None
        ran = execute(timed_argv(command, case["id"], out_step), timeout, core)
    return round(ran.seconds, TIME_PLACES) if ran.code == 0 else None


def rounded(x, places):
    return None if x is None else round(x, places)


# a case id no case has: a runner given it starts, finds nothing to build, and exits (SPEC.md)
NO_CASE = "harness/start"
# how many times a runner is started for what starting it costs, the median kept
STARTS = 5


def startup(k, timeout, one_core=None):
    """What starting kernel `k`'s runner costs, building nothing: the median of STARTS runs given
    NO_CASE as the timed run is given a case, of their time, CPU time and peak memory, and with
    `one_core` of their time on that core alone, after one start not counted (the first after a
    build reads its files from disk). None if the runner does not exit of itself in time."""
    def median(xs):
        xs = sorted(x for x in xs if x is not None)
        return xs[len(xs) // 2] if xs else None

    with tempfile.TemporaryDirectory() as tmp:
        argv = timed_argv(k["command"], NO_CASE,
                          os.path.join(tmp, "none.step") if k.get("step") is True else None)
        if execute(argv, min(timeout, 60)).code is None:
            return None
        runs = [execute(argv, min(timeout, 60)) for _ in range(STARTS)]
        if any(r.code is None for r in runs):
            return None
        out = {"seconds": round(median(r.seconds for r in runs), TIME_PLACES),
               "cpu_seconds": rounded(median(r.cpu for r in runs), TIME_PLACES),
               "peak_mb": rounded(median(r.peak_mb for r in runs), 1)}
        if one_core is not None:
            pinned = [execute(argv, min(timeout, 60), one_core) for _ in range(STARTS)]
            out["one_core_seconds"] = round(median(r.seconds for r in pinned), TIME_PLACES)
    return out


# the files a Python distribution installed, one a line: of the distribution named first
FILES_OF = ("import importlib.metadata as m, sys\n"
            "for f in m.files(sys.argv[1]) or []: print(f.locate())")


def binary(k):
    """The size of kernel `k`'s binary: the files its `binary` in kernels.json names, beside the
    language runtime its runner runs on and the system's own libraries. Each is a path or a glob
    pattern, relative to the repository (a directory counts every file under it), or a Python
    distribution, {"python": INTERPRETER, "distribution": NAME}, its files as that interpreter
    installed them. {"bytes", "files", "of"}, `of` the entries counted, or None where they name
    nothing there is (not built)."""
    found = set()
    for entry in k.get("binary") or []:
        if isinstance(entry, dict):
            try:
                p = subprocess.run([entry["python"], "-c", FILES_OF, entry["distribution"]],
                                   cwd=ROOT, capture_output=True, text=True, timeout=60)
            except (OSError, subprocess.TimeoutExpired):
                continue
            paths = p.stdout.splitlines() if p.returncode == 0 else []
        else:
            paths = glob.glob(os.path.join(ROOT, entry), recursive=True)
        for path in paths:
            if os.path.isdir(path):
                found.update(os.path.join(d, f) for d, _, fs in os.walk(path) for f in fs)
            elif os.path.isfile(path):
                found.add(path)
    # each file once, however it was named
    files = {os.path.realpath(f) for f in found}
    if not files:
        return None
    return {"bytes": sum(os.path.getsize(f) for f in files), "files": len(files),
            "of": k["binary"]}


def implemented(k, c):
    """Whether kernel `k` has an implementation of case `c` (or keeps none apart from its runner)."""
    return implements(k, c["id"])


def overhead(k, timeout, one_core):
    """Kernel `k`'s `startup` and the size of its `binary`, each printed as it is known."""
    start = startup(k, timeout, one_core)
    said = (f"{start['seconds'] * 1000:.1f} ms" if start else "not known: it did not exit in time")
    print(f"{k['name']:<14} starting its runner, building nothing: {said}", flush=True)
    size = binary(k)
    print(f"{k['name']:<14} its binary: " + (f"{size['bytes'] / 1e6:.1f} MB in {size['files']} "
          f"file{'s' if size['files'] != 1 else ''}" if size else "none found"), flush=True)
    return start, size


def step_said(row):
    """A result's STEP, as a run prints it: its verdict, and why where there is no file."""
    made = row.get("step")
    if not made:
        return ""
    why = made.get("refused") or made.get("error")
    return f"  STEP {step.verdict(made)}" + (f": {why[:70]}" if why else "")


def run_kernel(k, chosen, timeout, one_core=None):
    """Run `chosen` cases with kernel `k`: its rows, one per case, in case order. A row's time, CPU
    time and peak memory are its timed run's (`run_one`). With `one_core`, a core's number, each
    case it builds (passes, builds or answers wrong) has its timed run made again on that core
    alone for `one_core_seconds`."""
    writes_step = k.get("step") is True
    rows = []
    for c in chosen:
        keep = os.path.join(ROOT, "results", "meshes", k["name"], c["id"] + ".stl")
        keep_step = keep[:-len(".stl")] + ".step"
        for kept in (keep, keep_step):
            if os.path.exists(kept):
                os.remove(kept)
        if not implemented(k, c):
            outcome, why, reply, mesh, ran, made = ("missing", "no implementation", None, None,
                                                    None, None)
        else:
            outcome, why, reply, mesh, ran, made = run_one(k["command"], c, timeout, keep,
                                                           keep_step, writes_step)
        row = {
            "case": c["id"], "outcome": outcome, "why": why,
            "seconds": rounded(ran and ran.seconds, TIME_PLACES),
            "cpu_seconds": rounded(ran and ran.cpu, TIME_PLACES),
            "time_places": TIME_PLACES,  # rows from before microseconds have none: 3, milliseconds
            "peak_mb": rounded(ran and ran.peak_mb, 1),
            "measured": (reply or {}).get("measured", "exact"),
            "volume": (reply or {}).get("volume"), "area": (reply or {}).get("area"),
            "faces": (reply or {}).get("faces"), "mesh": mesh,
        }
        if made is not None:
            row["step"] = made
        if one_core is not None and outcome in BUILT:
            row["one_core_seconds"] = one_core_seconds(k["command"], c, timeout, one_core,
                                                       writes_step)
        rows.append(row)
        cost = (f"{ran.seconds:8.3f} s {ran.cpu if ran.cpu is not None else 0:8.3f} cpu "
                f"{ran.peak_mb or 0:7.1f} MB" if ran else " " * 33)
        print(f"{k['name']:<14} {outcome:<8} {c['id']:<44} {cost} {why[:70]}{step_said(row)}",
              flush=True)
    return rows


def version(k):
    if k.get("version"):
        try:
            return subprocess.run(k["version"], capture_output=True, text=True,
                                  cwd=ROOT, timeout=60).stdout.strip() or None
        except (OSError, subprocess.TimeoutExpired):
            pass
    return None


def tally(rows):
    out = {}
    for row in rows:
        out[row["outcome"]] = out.get(row["outcome"], 0) + 1
    return out


def spent(rows):
    """What a kernel's runs cost, in a line: their time and CPU time summed, their largest peak."""
    ran = [r for r in rows if r["outcome"] != "missing"]
    cpu = [r["cpu_seconds"] for r in ran if r.get("cpu_seconds") is not None]
    peak = [r["peak_mb"] for r in ran if r.get("peak_mb") is not None]
    out = f"{len(ran)} runs, {sum(r['seconds'] for r in ran):.1f} s"
    if cpu:
        out += f", {sum(cpu):.1f} s of CPU"
    if peak:
        out += f", at most {max(peak):.0f} MB"
    return out


@functools.cache
def host():
    """The machine a run is made on, since its times and memory are of that machine: its processor,
    how many cores the runs may use, its system, and the least peak memory a run can read there."""
    cpu = platform.processor() or platform.machine()
    try:
        with open("/proc/cpuinfo") as f:
            for line in f:
                if line.startswith("model name"):
                    cpu = line.split(":", 1)[1].strip()
                    break
    except OSError:
        pass
    cores = (len(os.sched_getaffinity(0)) if hasattr(os, "sched_getaffinity")
             else os.cpu_count())
    out = {"cpu": cpu, "cores": cores, "system": platform.platform(terse=True)}
    if MEASURED:
        # the peak of a process that holds next to nothing: the measurer's own, below which no run
        # can read (harness/measure.py)
        out["least_mb"] = round(execute([sys.executable, "-I", "-S", "-c", "pass"], 60).peak_mb, 1)
    return out


def record(k, rows, start, size, last=None):
    """Write results/<kernel>.json from its rows, of a run now or else of the `last` one, with its
    runner's `startup` (`start`) and the size of its binary (`size`)."""
    result = {"kernel": k["name"], "version": last["version"] if last else version(k),
              "measured": k.get("measured", "exact"),
              "run": last["run"] if last else time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
              "host": last.get("host") if last else host(),
              "startup": start, "binary": size,
              "tally": tally(rows), "cases": rows}
    with open(os.path.join(ROOT, "results", k["name"] + ".json"), "w") as f:
        json.dump(result, f, indent=1)
        f.write("\n")


def run_plan(plan, timeout, one_core=None):
    """Run what a plan from changed.py names, and fold it into each kernel's last results: their
    other rows are kept, and rows of cases that no longer exist are dropped (with their meshes)."""
    current = {c["id"]: c for c in cases()}
    registry = {k["name"]: k for k in kernels() if k.get("command")}
    for name, k in registry.items():
        path = os.path.join(ROOT, "results", name + ".json")
        last = {"cases": []}
        if os.path.exists(path):
            with open(path) as f:
                last = json.load(f)
        rows = {row["case"]: row for row in last["cases"]}
        wanted = [current[cid] for cid in plan["run"].get(name, []) if cid in current]
        gone = [cid for cid in rows if cid not in current]
        if not wanted and not gone:
            continue
        for cid in gone:
            del rows[cid]
            for ext in (".stl", ".step"):
                mesh = os.path.join(ROOT, "results", "meshes", name, cid + ext)
                if os.path.exists(mesh):
                    os.remove(mesh)
        # whether its runner is asked for anything: if not, it was not built (changed.py)
        asked = any(implemented(k, c) for c in wanted)
        # measured again when the kernel's whole suite runs (its runner changed) or they never
        # were, and the runner is to build something; a plan of a few cases keeps the last
        if asked and (len(wanted) == len(current) or not last.get("startup")):
            start, size = overhead(k, timeout, one_core)
        else:
            start, size = last.get("startup"), last.get("binary")
        rows.update({row["case"]: row for row in run_kernel(k, wanted, timeout, one_core)})
        # with nothing run (cases dropped, or only ones it has no implementation of), the runner
        # was not built: keep the last run's version
        record(k, [rows[cid] for cid in current if cid in rows], start, size,
               None if asked or "run" not in last else last)
        print(f"{name}: {tally(rows.values())}; {spent(rows.values())}")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--kernel", action="append", help="kernel name (repeatable)")
    ap.add_argument("--case", default="", help="run the cases whose id starts with this")
    ap.add_argument("--plan", help="run what this plan from changed.py names, and fold it into "
                    "the last results")
    ap.add_argument("--timeout", type=float, default=60)
    ap.add_argument("--one-core", action="store_true",
                    help="run each case a kernel builds again on one core alone, for its speedup "
                    "on all of them (Linux)")
    args = ap.parse_args()
    one_core = None
    if args.one_core:
        if not hasattr(os, "sched_setaffinity"):
            ap.error("--one-core needs a system that can pin a process to a core (Linux)")
        one_core = min(os.sched_getaffinity(0))
    os.makedirs(os.path.join(ROOT, "results"), exist_ok=True)
    if args.plan:
        with open(args.plan) as f:
            plan = json.load(f)
        if not plan["full"]:
            run_plan(plan, args.timeout, one_core)
            return
    chosen = cases(args.case)
    for k in kernels():
        if not k.get("command") or (args.kernel and k["name"] not in args.kernel):
            continue
        # a part of the suite is a check in hand, not a result: only a whole run is recorded, and
        # only it measures the runner's start-up and binary
        if not args.case:
            start, size = overhead(k, args.timeout, one_core)
        rows = run_kernel(k, chosen, args.timeout, one_core)
        if not args.case:
            record(k, rows, start, size)
        print(f"{k['name']}: {tally(rows)}; {spent(rows)}")


if __name__ == "__main__":
    main()
