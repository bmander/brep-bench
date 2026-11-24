#!/usr/bin/env python3
"""The brep-bench runner for geomsolver, whose solids are written in Solvent.

    python3 kernels/geomsolver/run.py CASE OUT.stl [OUT.step]   # CASE is an id, e.g. boolean/napkin-ring
    python3 kernels/geomsolver/run.py --version

A geomsolver part is a Solvent document, so each case is one: `cases/<category>/<name_snake>.sv`
beside this file states the solid the way a Solvent user writes it, and names it `part`. The runner
compiles it as a user does, with geomsolver's own compiler, `solventc` (built by build.py): the
document is parsed, elaborated and solved, and `part` is built by the core's B-rep kernel and
written as STEP (`solventc DOC --solid part --step OUT.step`). Given OUT.step, that file is the
whole answer, and nothing is measured or meshed.

Where the compiler declines to write the solid as STEP (its words name a solid it reads only point
by point, with no exact construction), the runner says so in `step_error` and compiles the
document again for the core's own measures of `part` (`--json --where part`: its volume and area)
and its triangles (`--stl OUT.stl`). Where that is declined too, the case is refused in the
compiler's words.

A document that does not parse, elaborate or solve is a fault of the implementation, not a
refusal: the runner prints the compiler's report and exits non-zero, so the harness records a
crash. A file with no code refuses: its paragraph of comments beginning `Refused:` says why.
"""
import json
import os
import re
import signal
import subprocess
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
SOLVENTC = os.path.join(HERE, "bin", "solventc")
# the solid each document names as its answer
PART = "part"
# a diagnostic's place, `path:line:col: `, which the reason leaves out
PLACE = re.compile(r"^\S+?:\d+:\d+: ")

if len(sys.argv) > 1 and sys.argv[1] == "--version":
    with open(os.path.join(HERE, "bin", "TAG")) as f:
        print(f"geomsolver {f.read().strip()} (solventc)")
    sys.exit(0)


def with_parent():
    """Ends solventc with the runner (Linux: PR_SET_PDEATHSIG), so that a runner the harness kills
    at its time limit leaves no compiler running on, to slow the cases after it."""
    if sys.platform.startswith("linux"):
        import ctypes
        ctypes.CDLL(None, use_errno=True).prctl(1, signal.SIGKILL)  # PR_SET_PDEATHSIG


class Fault(Exception):
    """The document itself is wrong: it did not parse, elaborate or solve."""


class Refused(Exception):
    pass


def pause():
    """Under the harness (`BREP_BENCH_WARM` set; SPEC.md, "A runner"), stops the runner until the
    harness continues it: once it has loaded and found its case, and once it has answered, so that
    the harness times the kernel alone, between the two."""
    if os.environ.get("BREP_BENCH_WARM") and hasattr(signal, "SIGSTOP"):
        sys.stdout.flush()
        os.kill(os.getpid(), signal.SIGSTOP)


def implementation(case_id):
    """The document implementing a case: cases/<category>/<name_snake>.sv."""
    category, name = case_id.split("/")
    path = os.path.join(HERE, "cases", category, name.replace("-", "_") + ".sv")
    if not os.path.exists(path):
        sys.exit(f"no implementation of {case_id}")
    return path


def refusal(source):
    """Why a file with no code refuses: its paragraph of comments that begins `Refused:`; None for
    a file with code."""
    lines = [line.strip() for line in source.splitlines()]
    if any(line and not line.startswith("//") for line in lines):
        return None
    words = [line[2:].strip() for line in lines]
    if "Refused:" not in " ".join(words):
        raise Fault("a file with no code must say why it refuses, in a paragraph beginning `Refused:`")
    start = next(i for i, w in enumerate(words) if w.startswith("Refused:"))
    paragraph = []
    for w in words[start:]:
        if not w:
            break
        paragraph.append(w)
    return " ".join(paragraph)[len("Refused:"):].strip()


def solved(stdout):
    """Whether solventc's report says the document solved: `DOC: solved` in its text, or, under
    `--json`, its document's solve a success."""
    if re.search(r": solved\b", stdout):
        return True
    try:
        return bool(json.loads(stdout)["documents"][0]["solve"]["success"])
    except (ValueError, KeyError, IndexError, TypeError):
        return False


def compile_(document, *outputs):
    """Run solventc over the document with these outputs. Returns its standard output where it
    succeeded; raises Refused, in the compiler's words, where the document solved but an output was
    declined, and Fault where the document itself failed."""
    run = subprocess.run([SOLVENTC, document, *outputs], capture_output=True, text=True,
                         preexec_fn=with_parent)
    if run.returncode == 0:
        return run.stdout
    if run.returncode != 1 or not solved(run.stdout):
        raise Fault(f"solventc exited {run.returncode}\n{run.stdout}{run.stderr}")
    errors = [PLACE.sub("", line) for line in run.stderr.splitlines() if "error[" in line]
    raise Refused("; ".join(errors) or run.stderr.strip() or "solventc declined the output")


def measured(document, out_stl):
    """The core's own measures of the part, and its triangles in out_stl. The triangles first:
    where solventc declines them too, it says so at once, before measuring a solid it reads only
    point by point, which can take minutes."""
    compile_(document, "--solid", PART, "--stl", out_stl)
    report = json.loads(compile_(document, "--json", "--where", PART))
    positions = report["documents"][0]["positions"]
    return {"status": "ok", "volume": positions[f"{PART}.volume"],
            "area": positions[f"{PART}.area"]}


def build(document, out_stl, out_step):
    if out_step:
        try:
            compile_(document, "--solid", PART, "--step", out_step)
            return {"status": "ok"}
        except Refused as why:
            declined = str(why)
        reply = measured(document, out_stl)
        reply["step_error"] = declined
        return reply
    return measured(document, out_stl)


def main():
    if len(sys.argv) not in (3, 4):
        sys.exit("usage: run.py CASE OUT.stl [OUT.step]")
    case_id, out_stl = sys.argv[1], sys.argv[2]
    out_step = sys.argv[3] if len(sys.argv) == 4 else None
    document = implementation(case_id)
    with open(document) as f:
        why = refusal(f.read())
    pause()
    # the runner's own clock, for the kernel's own time (SPEC.md, "A runner"): from the first
    # pause's end until the kernel has answered, its solid built and written or its refusal in
    # hand. The harness's, between the pauses, counts the stopping and continuing too
    since, answered = time.perf_counter(), None
    try:
        if why is not None:
            raise Refused(why)
        reply = build(document, out_stl, out_step)
    except Refused as why:
        answered = time.perf_counter()
        reply = {"status": "refused", "reason": str(why)}
    except Fault as fault:
        sys.exit(str(fault))
    reply["seconds"] = (answered or time.perf_counter()) - since
    print(json.dumps(reply))
    pause()


if __name__ == "__main__":
    main()
