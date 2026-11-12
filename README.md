# brep-bench

Rosetta Code for B-rep kernels. Each case describes a solid in plain words and gives its volume and
area in closed form where one exists. Each kernel builds the cases in its own language and idioms,
so reading one case across the kernels shows how each is used and where each falls short.

Results: **[brepbench.org](https://brepbench.org)** ([a guide to the
board](docs/board.md)).

## Why

People keep writing B-rep kernels from scratch, in Rust especially, many with an AI agent's help.
Each invents its own tests, so none can show how it compares with the others. brep-bench is a
shared yardstick: hard cases whose answers no single kernel chose, and every kernel in one table.

## Cases and grading

Over 300 cases, from textbook primitives, Booleans, sweeps and fillets to tangent contact,
self-intersecting curves, chamfers, splines, NURBS, quadrics and pinched tori, solid sweeps, lofts,
offsets, thickened sheets, shells, drafts, form features, singular points, direct edits, filled
surfaces, real parts and integration tests: the Utah teapot, a tugboat test print, and the
canonical #3DBenchy as a reference shape to come as near to as possible ([a tour](docs/cases.md)).

A runner builds the case and writes the solid as STEP with its kernel's own writer, and is timed
on that alone: a kernel's product is a B-rep, not a mesh. The harness then measures the file's solid
itself, untimed, from its exact geometry (its volume, area, centre of gravity and inertia), and it
**passes** when the file holds a closed solid, right side out, whose volume and area are within
1e-6 (relative) of the expected values. A kernel with no STEP to give writes its triangles as
STL and reports its own measures instead, and passes when they are within 1e-6 (1e-3 for a kernel
that can only measure its mesh) and the mesh is closed, consistently oriented, and encloses the
expected volume to within 2%.

A case with no closed form is *unsettled* until two independent kernels agree on it to 1e-8; until
then a sound result is graded **built**. A result that works around a missing feature (a ball
revolved from a half disk, for a kernel with no sphere) is marked † as a **workaround**; one that
stands out as especially clean, or for an idea in its kernel's API, is marked ✦ as **elegant**. A solid stored as a mesh in B-rep form, with many times the faces the
other kernels' solids have, is marked ▦ as **faceted**.
**Refused** ranks above **wrong**. Each STEP file is also judged (a closed B-rep, faceted, open, or
refused by the writer), and shown. The full rules are in [SPEC.md](SPEC.md).

## Running it

```bash
python3 harness/build.py         # build every kernel's runner
python3 harness/run.py           # run every case: results/*.json
cargo run --release --manifest-path harness/draw/Cargo.toml   # draw each case: site/drawings/
python3 harness/site.py          # render the board: site/
python3 -m http.server -d site   # serve it
```

The harness needs only Python 3.11+ (and, for the drawings' and the viewer's triangles of each STEP
file, Open CASCADE: `pip install cadquery-ocp`; and, to draw the cases, Rust); each runner needs its
kernel's toolchain. `build.py NAME` and
`run.py --kernel NAME` build and run one kernel. CI does not run on pull requests, so run your
changes locally. See [running the bench](docs/running.md) for more.

## Kernels

Open CASCADE (through Python), arris, cad-kernel, vcad, popkern, SolidCraft, Aetheris, next.BREP,
kentron, brepkit, solidcore, geop, cadrs and SMLib; monstertruck is listed but has no runner yet. See [the
kernels](docs/kernels.md) for links and details.

## Contributing

- **A kernel:** add `kernels/<kernel>/` with a runner (see [SPEC.md](SPEC.md#a-runner)), writing
  STEP too if your kernel can, and as many
  implementations as you like, following [the rules](SPEC.md#an-implementation), add it to
  `kernels.json`, and open a pull request. `kernels/occt`, `kernels/cad-kernel` and `kernels/geop`
  each have a file for every case to copy from. Where your kernel can't build a case, add the file
  anyway and have it refuse, saying why.
- **A case:** add `cases/<category>/<name>.md` with its closed forms, if it has them, and a
  description two careful readers would build the same way. Where it helps, end it with the solid
  [as a set](SPEC.md#the-solid-as-a-set). Hard cases are the most valuable,
  especially ones your own kernel gets wrong.

## License

The harness, the cases and the implementations here are MIT OR Apache-2.0, at your option. Each
kernel keeps its own license; an implementation only calls its kernel and does not include it.
