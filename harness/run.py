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

What is timed is a kernel's shortest path to a boundary representation, and no more. Each case is
run once: a kernel whose runner writes STEP (`"step": true` in kernels.json) is given OUT.step as
well as OUT.stl, and a STEP written is its whole answer: the runner neither measures nor meshes
the solid. The file is then graded, untimed: harness/step.py reads what it is made of, and the
harness measures its solid itself (harness/mass.py): its volume and area, graded to 1e-6, and its
centre of gravity and inertia, shown. Open CASCADE (harness/occt.py), where it is installed,
meshes the file for the drawings and the viewer; nothing it makes is graded. Where there is no
STEP (a kernel with no writer, or a writer that declined the solid), or the harness cannot measure
the file, the runner's own measures and triangles are graded, as they always were (where it wrote
a STEP, from a second run without OUT.step, untimed). The timed run's wall-clock time, the CPU
time it spent (user and system, over every thread) and its peak memory (the largest resident set)
are kept, from the operating system's account of the runner's process (wait4), which
harness/measure.py starts and waits for; nothing measured is graded. So is what starting the
runner costs, alone: it is given a case id no case has, `harness/start`, so that it starts and
builds nothing, a few times over (`startup`). With `--one-core` a case the kernel built is run a
second time on one core only, for what its other cores bought it; that run is timed and nothing
else. Times are kept to the microsecond: a kernel's own time, a run's less its runner's start-up,
can be under a millisecond. Standard library only (but for the pictures' Open CASCADE; where there
is no wait4, only the time is kept; pinning to one core needs Linux).
"""
import argparse
import concurrent.futures
import functools
import hashlib
import glob
import json
import os
import platform
import collections
import math
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


def off(key, got, exp):
    """How far a measure `got` is from the expected one, relative: the volume's and the area's by
    their own size; the centroid's distance by the solid's radius of gyration (from the expected
    volume and inertia, else the cube root of its volume); the inertia tensor's largest difference
    by its largest term. None where there is no measure."""
    if got is None:
        return None
    want = exp[key]["value"]
    if key in ("volume", "area"):
        return abs(got - want) / max(abs(want), 1e-12)
    if key == "centroid":
        v = exp["volume"]["value"]
        i = exp.get("inertia", {}).get("value")
        size = math.sqrt((i[0][0] + i[1][1] + i[2][2]) / (2 * v)) if i else abs(v) ** (1 / 3)
        return math.dist(got, want) / size
    return (max(abs(g - w) for gr, wr in zip(got, want) for g, w in zip(gr, wr))
            / max(abs(w) for wr in want for w in wr))


def missed(case, measures, rel, keys=("volume", "area", "centroid", "inertia"), whose=""):
    """Why the `measures` (a dict) fail the case's expected ones within `rel`: a list of reasons."""
    exp, why = case["expect"], []
    for key in keys:
        if key not in exp:
            continue
        d = off(key, measures.get(key), exp)
        if d is None or d > rel:
            got = measures.get(key)
            shown = f"{got:.12g}" if isinstance(got, float) else repr(got)
            want = exp[key]["value"]
            wanted = f"{want:.12g}" if isinstance(want, float) else repr(want)
            why.append(f"{key} {shown} against {wanted}{whose}"
                       + (f" ({d:.2g} off)" if d is not None and key not in ("volume", "area")
                          else ""))
    return why


def tolerance(case):
    """How near a result's measures must be to pass: the case's own tolerance, else EXACT."""
    return case.get("tolerance") or EXACT


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
    measures = {k: (mesh and mesh.get(k)) if from_mesh else reply.get(k)
                for k in ("volume", "area")}
    why += missed(case, measures, FROM_MESH if from_mesh else tolerance(case), ("volume", "area"))
    # the centroid and the inertia, the runner's own where it gives them, else its triangles'
    shape = {k: reply.get(k) if reply.get(k) is not None and not from_mesh else (mesh or {}).get(k)
             for k in ("centroid", "inertia")}
    exact = not from_mesh and all(reply.get(k) is not None for k in ("centroid", "inertia"))
    why += missed(case, shape, tolerance(case) if exact else FROM_MESH, ("centroid", "inertia"))
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


# the outcomes of a run that built a solid: it is run again on one core
BUILT = ("pass", "built", "wrong")


def last_json(text):
    """The last line of a program's output as a JSON object, or None."""
    try:
        reply = json.loads(text.strip().splitlines()[-1])
    except (ValueError, IndexError):
        return None
    return reply if isinstance(reply, dict) else None


def said(ran):
    """A runner's reply, the last line of its standard output as JSON, or None."""
    return last_json(ran.stdout)


def argv(command, cid, out_stl, out_step):
    """A run's command line: the case, OUT.stl, and OUT.step where the kernel's runner writes STEP
    (`out_step`, else None)."""
    return command + [cid, out_stl] + ([out_step] if out_step else [])


# the harness's own measurer of a STEP file's solid, run in a process of its own (it is given a
# time limit, as a run is)
MASS = os.path.join(os.path.dirname(os.path.abspath(__file__)), "mass.py")


def mass_of(out_step, timeout):
    """The mass properties of the solid in the STEP file at `out_step` (harness/mass.py):
    {"volume", "area", "centroid", "inertia", ...} (and `by_surface`, the solid with its loops
    taken by their surfaces' normals, where that differs), or {"invalid": why} where the file
    holds no solid, or {"error": why} where it cannot measure the file. Untimed."""
    try:
        p = subprocess.run([sys.executable, MASS, out_step], capture_output=True, text=True,
                           timeout=timeout, cwd=ROOT)
    except subprocess.TimeoutExpired:
        return {"error": f"not measured in {timeout:g} s"}
    answer = last_json(p.stdout)
    if answer is None:
        said_ = (p.stderr.strip().splitlines() or ["no answer"])[-1]
        return {"error": f"harness/mass.py failed: {said_[:200]}"}
    return answer


# how long the harness gives itself to measure a STEP file before it falls back to the runner's own
# measures (`--mass-timeout`), apart from the time a runner is given: a file is rarely slow to
# measure, and one that is should not hold up the run for the case's whole limit
MASS_TIMEOUT = 30.0


def cores():
    """How many cores the run may use."""
    return len(os.sched_getaffinity(0)) if hasattr(os, "sched_getaffinity") else \
        (os.cpu_count() or 1)


# how many files are measured at once, once every case has run (`--jobs`): measuring is untimed,
# so it may use every core, but only after the timed runs, which it would disturb
JOBS = cores()


class MassCache:
    """What harness/mass.py made of each STEP file a kernel wrote, by the file's SHA-256:
    results/mass/<kernel>.json, kept from run to run (CI keeps it in a cache of its own), so that
    a file a kernel writes again byte for byte is not measured again. It holds only what this
    measurer, with this time limit, made of the files: a change to harness/mass.py or
    harness/step.py, or to the limit, starts it afresh."""

    def __init__(self, kernel, limit=MASS_TIMEOUT, jobs=JOBS):
        self.limit, self.jobs = limit, jobs
        self.path = os.path.join(ROOT, "results", "mass", kernel + ".json")
        digest = hashlib.sha256(f"limit {limit:g}\0".encode())
        for name in ("mass.py", "step.py", "stl.py"):
            with open(os.path.join(os.path.dirname(MASS), name), "rb") as f:
                digest.update(f.read())
        self.version = digest.hexdigest()
        self.files = {}
        try:
            with open(self.path) as f:
                kept = json.load(f)
            if kept.get("version") == self.version:
                self.files = kept.get("files", {})
        except (OSError, ValueError):
            pass
        self.hits = 0
        self.used = set()

    def measure(self, paths):
        """Each of `paths` measured (`mass_of`), from the cache where it has it, the rest `jobs` at
        once, and Open CASCADE's triangles of each measured written beside it, its name's .step
        made .stl, for the pictures (`triangles`): {path: measures}."""
        def one(p):
            with open(p, "rb") as f:
                k = hashlib.sha256(f.read()).hexdigest()
            self.used.add(k)
            got = self.files.get(k)
            if got is None:
                got = self.files[k] = mass_of(p, self.limit)
            else:
                self.hits += 1
            if "volume" in got and got.get("area"):
                triangles(p, p[:-len(".step")] + ".stl", abs(got["volume"]) / got["area"])
            return got
        with concurrent.futures.ThreadPoolExecutor(max(1, self.jobs)) as pool:
            return dict(zip(paths, pool.map(one, paths)))

    def save(self, prune=False):
        """Write it; with `prune`, after a run of every case, holding only the files that run
        wrote, so that it does not grow without end."""
        files = {k: v for k, v in self.files.items() if k in self.used} if prune else self.files
        os.makedirs(os.path.dirname(self.path), exist_ok=True)
        with open(self.path, "w") as f:
            json.dump({"version": self.version, "files": files}, f, separators=(",", ":"))


# harness/occt.py, which meshes a STEP file for the pictures, and how long it may take over one:
# its triangles are not graded, and a file it is slow over goes without them
OCCT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "occt.py")
PICTURE_TIMEOUT = 15.0
# Open CASCADE could not be loaded: no triangles of STEP files this run
no_occt = False


def triangles(out_step, out_stl, thickness):
    """Open CASCADE's triangles of the solid in the STEP file at `out_step`, written to `out_stl`
    (harness/occt.py), as finely as its `thickness` (volume over area) asks, for the drawings and
    the viewer: whether it wrote them. Untimed, not graded, in a process of its own; where Open
    CASCADE is not installed, there are none."""
    global no_occt
    if no_occt:
        return False
    try:
        p = subprocess.run([sys.executable, OCCT, out_step, out_stl, repr(thickness)],
                           capture_output=True, text=True, timeout=PICTURE_TIMEOUT, cwd=ROOT)
    except subprocess.TimeoutExpired:
        return False
    if "No module named 'OCP'" in p.stderr:
        no_occt = True
        print("harness/occt.py: Open CASCADE (cadquery-ocp) is not installed, so STEP files "
              "are not meshed for the drawings and the viewer", file=sys.stderr)
        return False
    answer = last_json(p.stdout)
    return answer is not None and "error" not in answer


# how harness/step.py's verdict of a STEP file that is no closed B-rep reads, as a wrong answer
NOT_A_SOLID = {"open": "its STEP's faces do not close into a solid",
               "triangles": "its STEP holds triangles, and no B-rep",
               "empty": "its STEP holds no solid and no faces"}


# why a STEP that holds the solid only with its loops taken against ISO 10303-42 fails
BY_SURFACE = ("its STEP's loops run anticlockwise about their surfaces' normals where same_sense is "
              "false, not about the faces' normals as ISO 10303-42 directs: read by the standard, "
              "the faces are the other sides of their loops (read its way, it holds the solid)")


def grade_step(case, measured):
    """The outcome of a run whose solid was written as STEP and measured by the harness
    (`measured`, harness/mass.py's answer; SPEC.md, "Grading"): pass (built for an unsettled
    case) or wrong."""
    if measured["volume"] < 0:
        return "wrong", ("its STEP's solid is inside out: its faces face into it (its volume "
                         f"{measured['volume']:.10g})")
    exp = case["expect"]
    if exp is None:
        return "built", ""
    why = missed(case, measured, tolerance(case), whose=" (its STEP's)")
    return ("pass", "") if not why else ("wrong", "; ".join(why))


# the largest STEP file kept for the viewer, in bytes: a faceted solid's runs to tens of
# megabytes, which the results' cache and the site would carry for every run; its make-up is kept
# whatever its size
STEP_KEPT = 5_000_000


def keep_file(path, keep):
    os.makedirs(os.path.dirname(keep), exist_ok=True)
    shutil.copyfile(path, keep)


def graded(case, ran, out_stl, keep, timeout):
    """A run graded on the runner's own reply and triangles, as a kernel with no STEP is:
    (outcome, why, reply, mesh)."""
    if ran.code is None:
        return "timeout", f"over {timeout} s", None, None
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
        keep_file(out_stl, keep)
    return (*grade(case, reply, mesh), reply, mesh)


# what a case's timed run gave: its outcome (None while its STEP waits to be measured), why, the
# measures and how they were taken, its triangles' check, how it ran, its STEP's make-up, and the
# copy of its STEP waiting to be measured (or None)
First = collections.namedtuple("First", "outcome why reply mesh ran made pending")


def first_run(command, case, timeout, keep, keep_step, writes_step, work):
    """The timed run of one case (SPEC.md, "A runner"), and what can be graded of it at once
    (`First`). Where the runner wrote its solid as STEP and the file holds a closed solid, the
    file is kept in `work` to be measured once every case has run (`settle`); else the case is
    graded now, on the runner's own reply and triangles where it wrote no STEP."""
    with tempfile.TemporaryDirectory() as tmp:
        out_stl = os.path.join(tmp, "result.stl")
        out_step = os.path.join(tmp, "result.step") if writes_step else None
        ran = execute(argv(command, case["id"], out_stl, out_step), timeout)
        reply = said(ran) if ran.code == 0 else None
        if not (out_step and os.path.exists(out_step) and reply
                and reply.get("status") == "ok"):
            # no STEP: the kernel has no writer, or its writer declined the solid (`step_error`),
            # and the runner wrote its own triangles in its place; or the run failed
            outcome, why, reply, mesh = graded(case, ran, out_stl, keep, timeout)
            made = None
            if out_step and reply and reply.get("status") == "ok":
                made = ({"refused": str(reply["step_error"])} if reply.get("step_error") else
                        {"error": "no STEP written, and no step_error saying why"})
            return First(outcome, why, reply, mesh, ran, made, None)
        # the STEP is the answer: read for what it is made of now, measured later, untimed
        try:
            made = step.read(out_step)
        except (ValueError, OSError) as e:
            return First("wrong", f"its STEP cannot be read: {e}", None, None, ran,
                         {"error": f"unreadable STEP: {e}"}, None)
        if os.path.getsize(out_step) <= STEP_KEPT:
            keep_file(out_step, keep_step)
        verdict = step.verdict(made)
        if verdict in NOT_A_SOLID:
            return First("wrong", NOT_A_SOLID[verdict], None, None, ran, made, None)
        pending = os.path.join(work, case["id"].replace("/", "--") + ".step")
        shutil.copyfile(out_step, pending)
        return First(None, "", None, None, ran, made, pending)


def settle(command, case, timeout, keep, made, pending, measured):
    """The grade of a case whose STEP was measured (`measured`, with `by_surface` its loops taken
    by their surfaces' normals, where that differs): (outcome, why, measures, mesh).
    Where the harness cannot measure the file, the runner's own measures and triangles are graded
    instead, from a second run without OUT.step, untimed; else Open CASCADE's triangles of the file
    are kept for the pictures."""
    if "invalid" in measured:
        return "wrong", f"its STEP is no solid: {measured['invalid']}", None, None
    other = measured.get("by_surface") or {}
    if ("error" in measured or grade_step(case, measured)[0] == "wrong") and "volume" in other \
            and grade_step(case, other)[0] != "wrong":
        # read as its writer means it, the file holds the solid: that is why it fails
        return "wrong", BY_SURFACE, {"status": "ok", "measured": "step", **other}, None
    with tempfile.TemporaryDirectory() as tmp:
        out_stl = os.path.join(tmp, "result.stl")
        if "error" in measured:
            made["unmeasured"] = measured["error"]
            again = execute(command + [case["id"], out_stl], timeout)
            return graded(case, again, out_stl, keep, timeout)
        outcome, why = grade_step(case, measured)
        mesh = None
        pictured = pending[:-len(".step")] + ".stl"  # Open CASCADE's, made beside the measures
        if os.path.exists(pictured):
            try:
                mesh = stl.check(stl.read(pictured))
                keep_file(pictured, keep)
            except (ValueError, OSError):
                mesh = None
        measures = {k: v for k, v in measured.items() if k != "by_surface"}
        return outcome, why, {"status": "ok", "measured": "step", **measures}, mesh


# the decimal places a time in seconds is kept to: microseconds
TIME_PLACES = 6


def one_core_seconds(command, case, timeout, core, writes_step):
    """The wall-clock time of the case run again on `core` alone, or None if it ran out of time or
    failed. Nothing of the run but its time is kept."""
    with tempfile.TemporaryDirectory() as tmp:
        out_step = os.path.join(tmp, "result.step") if writes_step else None
        ran = execute(argv(command, case["id"], os.path.join(tmp, "result.stl"), out_step),
                      timeout, core)
    return round(ran.seconds, TIME_PLACES) if ran.code == 0 else None


def rounded(x, places):
    return None if x is None else round(x, places)


# a case id no case has: a runner given it starts, finds nothing to build, and exits (SPEC.md)
NO_CASE = "harness/start"
# how many times a runner is started for what starting it costs, the median kept
STARTS = 5


def startup(k, timeout, one_core=None):
    """What starting kernel `k`'s runner costs, building nothing: the median of STARTS runs given
    NO_CASE as a run is given a case, of their time, CPU time and peak memory, and with
    `one_core` of their time on that core alone, after one start not counted (the first after a
    build reads its files from disk). None if the runner does not exit of itself in time."""
    def median(xs):
        xs = sorted(x for x in xs if x is not None)
        return xs[len(xs) // 2] if xs else None

    with tempfile.TemporaryDirectory() as tmp:
        started = argv(k["command"], NO_CASE, os.path.join(tmp, "none.stl"),
                       os.path.join(tmp, "none.step") if k.get("step") is True else None)
        if execute(started, min(timeout, 60)).code is None:
            return None
        runs = [execute(started, min(timeout, 60)) for _ in range(STARTS)]
        if any(r.code is None for r in runs):
            return None
        out = {"seconds": round(median(r.seconds for r in runs), TIME_PLACES),
               "cpu_seconds": rounded(median(r.cpu for r in runs), TIME_PLACES),
               "peak_mb": rounded(median(r.peak_mb for r in runs), 1)}
        if one_core is not None:
            pinned = [execute(started, min(timeout, 60), one_core) for _ in range(STARTS)]
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


def run_kernel(k, chosen, timeout, one_core=None, mass_timeout=MASS_TIMEOUT, jobs=JOBS):
    """Run `chosen` cases with kernel `k`: its rows, one per case, in case order. Every case's timed
    run comes first, one after another, so that nothing else runs beside them; then the STEP
    files they wrote are measured, `jobs` at once, each in `mass_timeout` seconds (`MassCache`),
    and graded. With `one_core`, a core's number, each case it builds (passes, builds or answers
    wrong, or writes a STEP to be measured) is run again on that core alone for
    `one_core_seconds`."""
    writes_step = k.get("step") is True
    cache = MassCache(k["name"], mass_timeout, jobs) if writes_step else None
    work = tempfile.mkdtemp(prefix="brep-bench-")
    first = []
    try:
        for c in chosen:
            keep = os.path.join(ROOT, "results", "meshes", k["name"], c["id"] + ".stl")
            keep_step = keep[:-len(".stl")] + ".step"
            for kept in (keep, keep_step):
                if os.path.exists(kept):
                    os.remove(kept)
            if not implemented(k, c):
                got = First("missing", "no implementation", None, None, None, None, None)
            else:
                got = first_run(k["command"], c, timeout, keep, keep_step, writes_step, work)
            one = UNPINNED
            if one_core is not None and (got.outcome in BUILT or got.pending):
                one = one_core_seconds(k["command"], c, timeout, one_core, writes_step)
            first.append((c, keep, got, one))
            if got.pending:
                print(f"{k['name']:<14} {'ran':<8} {c['id']:<44} {cost_of(got.ran)} its STEP to be "
                      "measured", flush=True)
        # the STEP files measured, untimed, every core at work, and meshed for the pictures
        pending = [got.pending for _, _, got, _ in first if got.pending]
        if pending:
            print(f"{k['name']:<14} measuring {len(pending)} STEP files, {jobs} at once", flush=True)
        measured = cache.measure(pending) if pending else {}
        rows = []
        for c, keep, got, one in first:
            outcome, why, reply, mesh = got.outcome, got.why, got.reply, got.mesh
            if got.pending:
                outcome, why, reply, mesh = settle(k["command"], c, timeout, keep, got.made,
                                                   got.pending, measured[got.pending])
            rows.append(row_of(c, outcome, why, reply, mesh, got.ran, got.made, one))
            print(f"{k['name']:<14} {outcome:<8} {c['id']:<44} {cost_of(got.ran)} {why[:70]}"
                  f"{step_said(rows[-1])}", flush=True)
        if cache:
            if cache.hits:
                print(f"{k['name']:<14} {cache.hits} STEP files measured before, and not again",
                      flush=True)
            cache.save(prune=len(chosen) == len(cases()))
        return rows
    finally:
        shutil.rmtree(work, ignore_errors=True)


# a case not run again on one core (`one_core_seconds`'s None is a run that failed)
UNPINNED = object()


def cost_of(ran):
    return (f"{ran.seconds:8.3f} s {ran.cpu if ran.cpu is not None else 0:8.3f} cpu "
            f"{ran.peak_mb or 0:7.1f} MB" if ran else " " * 33)


def row_of(c, outcome, why, reply, mesh, ran, made, one):
    """A result's row, as results/<kernel>.json keeps it."""
    row = {
        "case": c["id"], "outcome": outcome, "why": why,
        "seconds": rounded(ran and ran.seconds, TIME_PLACES),
        "cpu_seconds": rounded(ran and ran.cpu, TIME_PLACES),
        "time_places": TIME_PLACES,  # rows from before microseconds have none: 3, milliseconds
        "peak_mb": rounded(ran and ran.peak_mb, 1),
        "measured": (reply or {}).get("measured", "exact"),
        "volume": (reply or {}).get("volume"), "area": (reply or {}).get("area"),
        "faces": (reply or {}).get("faces"), "mesh": mesh,
        # the centre of gravity and the inertia about it (unit density): of the STEP's solid
        # where the harness measured it, else of the triangles
        "centroid": (reply or {}).get("centroid") or (mesh or {}).get("centroid"),
        "inertia": (reply or {}).get("inertia") or (mesh or {}).get("inertia"),
    }
    if made is not None:
        row["step"] = made
    if one is not UNPINNED:
        row["one_core_seconds"] = one
    return row


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
    out = {"cpu": cpu, "cores": cores(), "system": platform.platform(terse=True)}
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


def run_plan(plan, timeout, one_core=None, mass_timeout=MASS_TIMEOUT, jobs=JOBS):
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
        rows.update({row["case"]: row for row in run_kernel(k, wanted, timeout, one_core,
                                                            mass_timeout, jobs)})
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
    ap.add_argument("--mass-timeout", type=float, default=MASS_TIMEOUT,
                    help="how long harness/mass.py may take over one STEP file before the "
                    "runner's own measures are graded instead (default %(default)g s)")
    ap.add_argument("--jobs", type=int, default=JOBS,
                    help="how many STEP files to measure at once, after the timed runs "
                    "(default: the cores the run may use, %(default)d)")
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
            run_plan(plan, args.timeout, one_core, args.mass_timeout, args.jobs)
            return
    chosen = cases(args.case)
    for k in kernels():
        if not k.get("command") or (args.kernel and k["name"] not in args.kernel):
            continue
        # a part of the suite is a check in hand, not a result: only a whole run is recorded, and
        # only it measures the runner's start-up and binary
        if not args.case:
            start, size = overhead(k, args.timeout, one_core)
        rows = run_kernel(k, chosen, args.timeout, one_core, args.mass_timeout, args.jobs)
        if not args.case:
            record(k, rows, start, size)
        print(f"{k['name']}: {tally(rows)}; {spent(rows)}")


if __name__ == "__main__":
    main()
