# brep-bench

Rosetta Code for B-rep (boundary representation) solid-modelling and CAD kernels. Each case
describes a solid in plain words and gives its volume and area as closed forms. Each kernel builds
every case in its own language and idioms, and every result is graded the same way. The results
are published side by side: **https://bmander.github.io/brep-bench/**

The board ranks no kernel above another, and lists them in the order of `kernels.json`. It has:

- an overview: each kernel's outcomes, each category's outcomes by kernel, and a strip of every
  case's outcome for every kernel;
- an explorer of every case, searched by words and filtered by category, tag, whether it is
  settled and each kernel's outcome, with views of the cases no kernel passes, the cases one kernel
  alone passes, and the cases two or more kernels differ on; what it shows is in its address;
- a page for each kernel: its outcomes by category, its refusals grouped by reason, its wrong
  answers, its workarounds by what it lacks, and its times;
- `board.json`, every outcome of every case in one file, which these pages read.

Each case has its own page with:

- the description;
- a drawing of the expected solid (three views and an isometric);
- a 3D viewer of every kernel's result, with any open edges of its mesh marked in red;
- each result's STL to download;
- every kernel's code for it, side by side.

## Why

People keep writing B-rep kernels from scratch, in Rust especially, many of them with an AI agent's
help. Each one invents its own tests, so none can show how it compares with the others. brep-bench
is the shared yardstick: one set of hard cases, one way of grading them, and every kernel in one
table. Pooling effort begins with knowing where each kernel stands.

There is no case language to translate: a case is a description, and each kernel's implementation
is written the way that kernel's users write. Reading one case across the kernels shows how each
one is used, and what each one cannot yet do.

## What it checks

The first 33 cases are textbook solids:

- **Primitives:** box, cylinder, cone, sphere, torus, extrusion, revolution.
- **Booleans:** coincident and partly coincident faces, tangencies, Steinmetz solids, spheres, tori,
  and a run of nine cuts.
- **Sweeps:** a circle along helices and arcs.
- **Fillets:** one edge, every edge of a box (three meeting at each corner), and a cylinder's rim.

Then come harder ones: NURBS surfaces in Booleans, tangent and near-tangent contact, fillets and
chamfers that consume faces, chamfers meeting at a corner mitred or cut flat, three or four to
it, splines, and **solid sweeps**: everything a tool passes through as it moves, from a ball
swung through a block (a closed form) to a wheel screwed along a rod, a rack rolled round a gear
blank and a torus tumbling through space, whose walls are envelopes no kernel
here can yet build.

Then come **lofts** beyond two sections joined edge to edge: a square lofted to an octagon and to
a circle, each corner of the square paired with a whole side or arc; rectangles whose corners run
along four Bézier guides, one pinched to 0.4 thick on its way from 24 × 6.4 to 6 × 24.4 and one
closing to a spire's point; and sections turned as they rise, along spiral and helical guides, a
square tapering through a half turn and a blade 40 × 0.4 twisted through a whole one. All but the
square-to-round have closed forms; its cones' area is an elliptic integral, which OCCT and
cad-kernel agree on.

Then **offsets**: solids grown or shrunk by a distance, to every point within that distance of
the solid or at least that far inside it. They run from a box grown or shrunk and a cylinder's
rims rounded, through concave edges kept sharp or rounded and the oblique corners of a tetrahedron
and a cone's apex, to offsets that cross themselves: a groove grown shut, a flange shrunk away,
and a torus grown past its hole into an apple.

Then **thickened sheets**: a sheet, a face or faces with no solid behind them, thickened into the
solid between it and its faces moved along their normals, to one side or to both. They run from a
flat rectangle and a plate with holes, through curved sheets whose rims sweep cones or cylinders
(a half pipe either way, a whole tube closed on its seam, a spherical cap, a cone's side, the
saddle of a torus's inner half) and a parabolic trough made as a B-spline, to sheets of several
faces: a bent flange, an angle and an open box thickened outside their creases with the corners
kept sharp or rounded, or inside them, where the moved faces cross. The hardest thicken a sheet to
a point, a hemisphere to its centre; close a sphere's skin into a hollow ball; and thicken a bend
past its own radius, so that the moved bend turns inside out and is no part of the solid.

Then **shells**: solids hollowed to a wall of constant thickness, open at some of their faces or
closed round a void. They run from a box open at its top, at two faces opposite or sharing an
edge, or at every face but one, through walls grown outward rounded or sharp and an L's inside
corner rounded or kept sharp, to the thickness at its limits: walls that meet at half the width
and hollow nothing, a slot 0.02 wide, a wall 0.001 thick, rounds shrunk to an edge or consumed,
a neck whose walls cross and fill it, and a bore's tube running into a wall. Curved walls bring a
cup, a bowl through its pole, a cone's apex moved in, a capsule open at a dome tangent to its
side, a torus hollowed and one halved into a pipe, a bore opened into a hollow block, a cylinder
open at its side that falls into two discs, and a ball cut by a plane whose cavity reaches it
aslant, through a pinhole, or not at all.

Then come real parts, whose scenarios are taken from the example documents of
[geomsolver](https://github.com/bmander/geomsolver) and written out in plain words: a flange, a
pulley, a tray, a star handle, a turned shaft, ducts swept along a bend. Most have closed forms;
where one has none, its answer is accepted only when two kernels, each built independently from the
case's words alone, agree. That agreement also shows the words describe one solid.

A case **passes** when three things hold:

- the kernel's volume and area are within 1e-6 of the closed forms;
- the triangles it writes are watertight and consistently oriented;
- the triangles bound the expected volume.

An implementation that gets there by working around a feature its kernel lacks (a ball turned
from a half disc where there is no sphere primitive, a sweep built from its faces where there is no
sweep of a solid) is flagged **workaround**: it is graded the same, but the board marks it †.

A kernel that **refuses** a case ranks above one that answers it **wrong**. A case with no
implementation yet is **missing**, which says nothing about the kernel. The full rules are in
[SPEC.md](SPEC.md).

## Layout

```
cases/<category>/<name>.md         a case: its closed forms (TOML header) and its description
kernels.json                       the kernels: how to build and run each, where its cases live
kernels/<kernel>/                  a kernel's runner and its implementation of each case
kernels/<kernel>/workarounds.toml  the cases it builds by working around a feature it lacks
harness/                           build, run and grade, draw, and render the site
harness/web/                       the board's pages, which read the site's board.json
results/<kernel>.json              the last whole run
```

## Running it

```bash
python3 harness/build.py           # every kernel's runner (cargo, pip, dotnet)
python3 harness/run.py             # every kernel, every case: results/*.json
python3 harness/draw.py            # each case's drawing: site/drawings/
python3 harness/site.py            # board.json, the board's pages and a page per case: site/
python3 -m http.server -d site     # the board reads board.json, so serve it to open it
```

`harness/run.py --kernel NAME --case boolean/` runs part of the suite. The harness uses only
Python's standard library (3.11 or later). The runners need their own toolchains: Rust, Python's
pip, and the .NET 10 SDK for Aetheris.

On GitHub, a push to `main` runs only what it changed (`harness/changed.py` decides: a case, a
kernel's implementation of one, or a kernel's runner), on top of the last run's results; a push
that changes no case skips the bench, building nothing. The site is published after every push.
The whole suite runs nightly, and when the harness's running or grading changes.

## Kernels

| kernel | its implementations |
|---|---|
| [Open CASCADE](https://github.com/Open-Cascade-SAS/OCCT) | `kernels/occt` (Python, through OCP), the reference |
| [arris](https://github.com/arris-labs/arris) | `kernels/arris` (Rust) |
| [cad-kernel](https://crates.io/crates/cad-kernel) | `kernels/cad-kernel` (Rust, on a release pinned in its `Cargo.toml`) |
| [truck](https://github.com/ricosjp/truck) | `kernels/truck` (Rust); measured from its mesh; frozen at the first 46 cases |
| [vcad](https://github.com/ecto/vcad) | `kernels/vcad` (Rust); measured from its mesh |
| [popkern](https://github.com/popkern-dev/popkern) | `kernels/popkern` (Rhai, popkern's scripts, run by a Rust runner) |
| [SolidCraft](https://solidcraft.io) | `kernels/solidcraft` (Rust, on SolidCraft's kernel crate, solidcraft-kernel); measured from its mesh |
| [Aetheris](https://github.com/yuechen-li-dev/Aetheris) | `kernels/aetheris` (Firmament, Aetheris's modelling language, compiled by a C# runner); measured from its mesh |
| [monstertruck](https://github.com/virtualritz/monstertruck), [kentron](https://github.com/rsz269/kentron), [brepkit](https://github.com/andymai/brepkit), [next.BREP](https://github.com/mmiscool/next.BREP.io_RUST_BREP_KERNEL) | wanted |

## Contributing

- **A kernel:** add `kernels/<kernel>/` with a runner (it is given a case id and an output path,
  and prints one line of JSON; see [SPEC.md](SPEC.md#a-runner)) and an implementation of as many
  cases as you like, then add the kernel to `kernels.json`. Unwritten cases show as missing. If
  your kernel cannot do something, write the case anyway and refuse with the reason: that is an
  honest result.
- **A case:** add `cases/<category>/<name>.md` with its closed forms and a description that two
  careful readers would build the same way. Hard cases are the most valuable, especially ones
  your own kernel gets wrong.
- **A better implementation:** welcome when it is more idiomatic, or when it builds what the
  description asks where the old one refused.

## Licence

The harness, the cases and the implementations written here are MIT OR Apache-2.0, at your option.
Each kernel keeps its own licence. An implementation only calls its kernel; it does not include it.
