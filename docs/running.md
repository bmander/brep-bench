# Running the bench

## Locally

```bash
python3 harness/build.py           # every kernel's runner (cargo, pip, dotnet)
python3 harness/run.py             # every kernel, every case: results/*.json
cargo run --release --manifest-path harness/draw/Cargo.toml   # each case's drawing: site/drawings/
python3 harness/site.py            # board.json, the board's pages and a page per case: site/
python3 -m http.server -d site     # the board reads board.json, so serve it to open it
```

`harness/build.py NAME` builds one kernel's runner, and `harness/run.py --kernel NAME --case
boolean/` runs part of the suite; `NAME` is the kernel's `name` in `kernels.json` (e.g. `occt`). A
run narrowed by `--case` is only a check and is not recorded; a run of every case, even for one
kernel, writes `results/<kernel>.json`.

Each case is run once. A kernel marked `"step": true` in `kernels.json` writes its solid as STEP
and nothing more ([SPEC.md](../SPEC.md#step-is-the-answer)); the harness then reads the file,
untimed: `harness/step.py` for what the solid is made of, and `harness/mass.py` for its volume
and area, which are graded, and its centre of gravity and inertia, which are shown (`python3
harness/mass.py FILE` prints them for any STEP file). Open CASCADE (`harness/occt.py`, where
`cadquery-ocp` is installed) meshes the file for the drawings and the viewer. A kernel with no STEP
to give, or a file the harness cannot measure, is graded on the runner's own measures and
triangles. The files are measured once all of a kernel's timed runs are done (so that nothing runs
beside those), as many at once as the run may use cores (`--jobs`), each in `--mass-timeout`
seconds (30 by default) and its pictures' meshing in 15; and what each file measured is kept by
its SHA-256 in `results/mass/<kernel>.json`, so that a file a kernel writes again byte for byte is
not measured again (CI keeps that file from run to run in a cache of its own; a change to
`mass.py` or `step.py` starts it afresh). So the time, CPU time (user and system, over every thread) and peak memory recorded
beside each outcome are a kernel's shortest path to a B-rep, not its meshing; none of them is
graded. Runners are started by
`harness/measure.py`, a small process that holds almost no memory, because on Linux a child's
peak resident set starts at its parent's. Before its cases, each runner is given the case id
`harness/start`, which names no case, five times; the median is the kernel's start-up, which the
board subtracts from its times. Times are recorded to the microsecond, since a fast kernel's run
less its start-up can be under a millisecond. In its means and ratios the board takes any time under 0.1 ms, about the
spread of a runner's start-up, as 0.1 ms; a time recorded only to the millisecond, or one whose
kernel's start-up was never measured, it takes as no less than 1 ms. A CI run of only some of a
kernel's cases keeps its last start-up and binary size. `--one-core` runs each case a kernel
builds a second time, pinned to one core, for its speedup on all of them (Linux only). For each
case a kernel with `"step": true` builds, `harness/step.py` reads the file for what the solid is made of (its
form, its topology, its faces by surface and edges by curve, its unpaired edges), and the result keeps that
as `step`, or `{"refused": why}` where the kernel's writer declined it, or `{"error": why}` where
the run failed. The run prints each one's verdict (`STEP brep`, `STEP refused: …`) at the end of
its line. The file
is kept beside the triangles, as `results/meshes/<kernel>/<case>.step`, for the case page's
viewer, if it is no larger than 5 MB: a faceted solid's can run to tens of megabytes (`python3
harness/step.py FILE` prints the make-up and its verdict for any STEP file). Each
result file also records its machine and the size of the kernel's binary, as `binary` in
`kernels.json` names it.

CI does not run on pull requests, so run a change locally before proposing it.

The harness uses only Python's standard library (3.11 or later); Open CASCADE (`python3 -m pip
install cadquery-ocp`) is wanted only for the triangles of the STEP files that the drawings and
the viewer show. The cases are drawn by a Rust program, `harness/draw` (given case ids, or what
they start with, `-- boolean/`, it draws only those; `cargo test --manifest-path
harness/draw/Cargo.toml` runs its tests). The runners need their
own toolchains: Rust, Python's pip and, for Aetheris, the .NET 10 SDK.

## On GitHub

A push to `main` reruns only the cases its changes affect, on top of the last run's results.
`harness/changed.py` decides:

- a case: every kernel runs it;
- a kernel's implementation of a case: that kernel runs that case;
- anything else under `kernels/<kernel>/` (except its `workarounds.toml` and `elegance.toml`), or
  how its entry in `kernels.json` builds or runs it: that kernel runs every case;
- a kernel newly added, or one with no results yet that the push touches: it runs every case;
- a change of which kernel is the reference: every case is drawn again;
- a deleted case: its rows are dropped from the results;
- how a run is made, measured or graded (`harness/run.py`, `measure.py`, `mass.py`, `occt.py`,
  `stl.py`, `step.py`, `suite.py`, `build.py`): the whole suite runs;
- `harness/draw/`: every case is drawn again;
- anything else (the site, the docs, a kernel's display-only fields, such as its license):
  nothing runs, and the site is only rebuilt.

With no last results to build on, a push runs the whole suite. The site is published after every push.

Each kernel to build runs in a job of its own, all at once, so a run takes as long as its slowest
kernel, not as long as all of them. Each job builds its kernel, runs its cases on top of the last
results and hands on `results/<kernel>.json` and its triangles; a last job gathers them, records
what needs no runner (removed cases, cases a kernel has no implementation of), draws the cases and
keeps the results for the next run. If any kernel's job fails, nothing is kept, and the next run
starts again from the last results; the site still shows them.

A run whose kernels all ran but whose last job failed (so its results were never kept) need not be
run again while its artifacts last (three days): a manual dispatch given its id as `results_from`
runs no kernel, gathers that run's results, draws them and keeps them under the commit they were
made at, so the next push plans from there. It refuses a run with a kernel's job that failed or
whose results are gone.

The whole suite also runs nightly, and on a manual dispatch unless its `full` input is unchecked. CI
allows 120 s per case, not the 60 s default, so a timeout on the board means more than 120 s. CI
runs with `--one-core`, which roughly doubles the time spent running cases.

## Layout

```
cases/<category>/<name>.md         a case: its closed forms (TOML header) and its description
kernels.json                       the kernels: how to build and run each, where its cases live
kernels/<kernel>/                  a kernel's runner and its implementation of each case
kernels/<kernel>/workarounds.toml  the cases it builds by working around a feature it lacks
kernels/<kernel>/elegance.toml     its implementations that stand out as elegant, and why
harness/                           build, run and grade, draw, and render the site
harness/draw/                      the program that draws the cases (Rust)
harness/web/                       the board's pages, which read the site's board.json, and its icons
results/<kernel>.json              the last results
docs/                              these notes
```
