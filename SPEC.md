# brep-bench specification, version 1.16

brep-bench works like Rosetta Code. Each **case** describes a solid in plain words and gives its
volume and area as closed forms. Each **kernel** has its own **implementation** of each case,
written the way that kernel's users write: in its own language, with its own API and idioms. A
**runner** builds the case with the kernel and reports what it made. The **harness** grades every
result the same way.

## Principles

- **No canonical script.** A case is a description of a solid, not code. There is no case language
  for a kernel to translate, and no implementation is privileged. Reading one case's
  implementations side by side shows how each kernel is used.
- **Neutral answers.** Every expected value comes from outside any one kernel, and `exact` says
  where it came from: a closed form, written beside the number, where one is cheap to derive and
  check; otherwise at least two independent kernels, each built from the description alone, that
  agree to 1e-8 on both measures (the value is their mean). Every result is also checked by the
  harness itself: its STEP file read and measured, or, for a kernel with no STEP to give, its own
  triangles, watertight, consistently oriented, bounding the expected volume.

  No answer ever comes from a single kernel.
- **The hardest cases are kept even without an answer.** A case that has no closed form and that no
  two kernels yet agree on is *unsettled* (`settled = false` in its header, and no `[volume]` or
  `[area]`). It still describes one definite solid. The board shows what each kernel builds for
  it (its measures, whether its mesh is sound, the solid in 3D) and which kernels agree with which.
  Once two kernels agree to 1e-8, the case can be settled with their mean.
- **The same rules for every kernel.** The rules change only by a versioned edit to this file,
  made before any results under the new rules are published.
- **A refusal ranks above a wrong answer.** A kernel that says "I cannot" is more useful than one
  that returns the wrong solid. A case with no implementation yet is *missing*, which is neither.

## A case

A case is a Markdown file, `cases/<category>/<name>.md`. Its id is the path without the extension,
for example `boolean/napkin-ring`. A TOML header between `+++` lines holds what is graded; the
text after it describes the solid.

```
+++
title = "A napkin ring"
tags = ["boolean", "cut", "analytic"]

[volume]
value = 268.082573106329
exact = "πh³/6, h = 2√(5² − 3²) = 8"
tex = '\tfrac{\pi h^3}{6},\quad h = 2\sqrt{5^2 - 3^2} = 8'

[area]
value = 402.1238596594935
exact = "2πRh + 2πrh"
tex = '2\pi R h + 2\pi r h'
+++

A ball of radius 5 centered on the origin, with a hole of radius 3 bored right through it along the
z axis.
```

`tex` is the same closed form in LaTeX, which the case's page typesets; it is left out where there
is no closed form. It is a TOML literal string (in single quotes), so its backslashes stay as written.

The header may also say where a case came from: `origin = "geomsolver rust/examples/solid_flange.sv"`.

The description is Markdown, which the case's page renders (`harness/markdown.py`): paragraphs,
**bold** and *emphasis*, `code`, lists, tables, links, and math in LaTeX, `$P_{ij}$` in a line and
`$$S(u, v) = \sum_i \sum_j B_i(u) B_j(v) P_{ij}$$` standing alone, which KaTeX typesets as it does
the closed forms. A dollar meant as a dollar is written `\$`. Plain words and Unicode (πr², x ≤ 3)
are as good where they read clearly; math is for what they cannot say well, such as sums and
subscripts.

How a description is written:

- **Coordinates** are right-handed x, y, z, with z up. Lengths have no unit. Angles are in degrees,
  and a turn about an axis is right-handed unless the text says otherwise.
- **It names the solid and where it lies.** Placement is not graded (volume and area do not depend
  on it), but it keeps every kernel's result in the same place, so the drawings and the 3D viewer
  can compare them.
- **It says what to build, not how.** Hints that keep a construction away from an accidental
  degeneracy are allowed ("if you cut the hole with a cylinder, make it run past both faces"). So is
  saying which operation a case is about ("drill them one after another, each hole its own cut").
- **It defines its solid directly.** A case states its solid by well-defined geometry and
  parameters and the operations that make a solid of them (primitives, sweeps, lofts, Booleans,
  blends, offsets and the like), applied reasonably directly: not as a path through other solids
  that a direct construction of the same result would skip, such as a solid built only so that
  some of its faces can be deleted or given new surfaces. The grading sees only the result, so a
  description that must forbid the direct construction ("do not use a box") to keep its case
  meaningful is asking about a path the grading cannot see, and is not a case. The geometry a
  solid is bounded by is part of its definition, not a path: a ball bounded by one NURBS face is
  not a sphere primitive. The one exception is the direct edits of `edit/`, which move a finished
  solid's faces by a distance: each states the solid it starts from, which faces move and how far,
  and the solid that results.
- **It must be unambiguous enough to have one answer.** If two careful readers could build
  different solids from it, that is a bug in the case.
- **Where blends meet at a corner, it says what the corner is, as a composition.** Chamfers or
  fillets meeting at a vertex can close it more than one way: running on until they meet in a
  miter, or cut off by a corner face of their own. The words of one kernel's blend are no
  definition, so the description says which, and states the solid as a composition of operations
  that have one answer: the solid cut by the chamfers' planes, and by each corner's plane where the
  corner is cut flat (`chamfer/cube-all-edges`); and where the case is about a miter, the edges
  chamfered one after another, each its own chamfer ending on the faces at its ends
  (`chamfer/cube-all-edges-mitered`). A corner is
  described only where that composition is one solid. A flat corner face through the corners the
  faces are left with nearest a vertex is one plane where three faces meet, but where four or more
  do only when those corners happen to lie in a plane (the octahedron's, by its symmetry, and not
  a pyramid's apex on a rectangle), so no case asks for a flat corner where they do not.
- **A shell says what its wall's inside is.** A wall of constant thickness can be made more than
  one way where the faces moved to make its inside part or cross. The description says, case by
  case: at a concave edge, whether the moved faces are extended until they meet (sharp) or the
  wall is every point within the thickness of the faces (a round about the edge); where an opening
  meets a wall, that the moved face runs on to the opening's surface, which is the rim; and where
  the thickness consumes a face, closes a gap or never reaches an opening, what is left. It then
  states the result exactly, as a composition with one answer: the solid less its cavity, given
  as a solid, or for a wall grown outward the solid grown less the solid.

### The solid as a set

A description may end with a paragraph headed `**As a set:**` and one formula in display math: the
solid again, as the set of its points. It is optional, and it is not a script. It names no
operation of any kernel and no order of construction, only which points are in the solid, so it
stays within "no canonical script": there is nothing in it to translate, and an implementation
still builds from the description. The words are the definition. The formula restates them, and
where the two differ that is a bug in the case. It is not graded.

It is worth writing where it is shorter than the words or settles something they leave to care:
which side, which corner, which of two readings. And since a set is a test of whether a point is
in the solid, it gives any reader, a person or a program, an estimate of the volume that owes
nothing to a kernel. It is left out where the case is about what bounds the solid and not which
points it holds (a ball whose one face is a NURBS is the same set as a ball), and where the
formula would be longer or harder to read than the description.

What a formula may use, written in LaTeX in the case and shown here in plain symbols:

- `p = (x, y, z)` is a point, `ρ = √(x² + y²)` its distance from the z axis, and `φ` its angle
  about the z axis from +x toward +y. `‖p‖` is its distance from the origin.
- `{ conditions }` is every point that meets the conditions: `{ ρ ≤ 5, 0 ≤ z ≤ 10 }` is a
  cylinder. A condition that leaves a coordinate out leaves it free, so `{ ρ ≤ 3 }` is the whole
  cylinder about the z axis, without ends.
- The solid is the closure of the interior of the set written. So `≤` and `<` say the same thing,
  a difference leaves no skin behind, and a part with no volume (a face, a slit, a point) is no
  part of the solid.
- `[a, b] × [c, d] × [e, f]` is a box, and `A × [e, f]` the plane region `A` of x and y extruded
  from z = e to z = f. `poly(…)` is the polygon through the points given, in order, and
  `conv(…)` the convex hull of what is given.
- `B_r(c)` is the ball of radius r about c, and `B_r` the one about the origin. `d(p, A)` is the
  distance from p to the nearest point of A, which may be a line, a curve, a region or a solid;
  `d((x, y), A)` is the same in the plane.
- `∪`, `∩` and `∖` are union, common part and difference, and `⋃` a union over everything its
  index runs through, a continuous interval included.
- `A ⊕ B = { a + b : a ∈ A, b ∈ B }`. So `A ⊕ B_r` is A grown by r, every point within r of it,
  and `A ⊕ { s·v : 0 ≤ s ≤ 1 }` is everything A passes through as it slides along v.
  `A ⊖ B_r = { p : B_r(p) ⊆ A }` is A shrunk by r. Then `(A ⊖ B_r) ⊕ B_r` is the union of every
  ball of radius r inside A: A with its convex edges and corners rounded to r.
- `R_ℓ(θ)` is the turn by θ about the line ℓ, right-handed, and `R_z` the turn about the z axis.
  Motions are applied right to left: `R_B(−t) R_A(t/4) T` is T turned about A and then about B.
  `⋃ M_t T` over t in an interval is everything T passes through as it moves.
- A letter may stand for a set or a line the description has named (K the solid started from, A
  and B two axes), given after the formula or left to the words where they are plain.

## An implementation

Each kernel lives in `kernels/<kernel>/`, listed in `kernels.json`. The directory holds a runner and
one source file per case. The file's path comes from the kernel's `source` pattern in
`kernels.json`, for example `kernels/arris/src/cases/{category}/{name_snake}.rs`. The pattern's
fields:

- `{case}`: the id, `boolean/napkin-ring`;
- `{category}`: `boolean`;
- `{name}`: `napkin-ring`;
- `{name_snake}`: `napkin_ring`.

An implementation builds the described solid the way a user of its kernel would:

- **It may** use anything in the kernel's public API. It may compose the kernel's operations (a
  sphere as a revolved half disk, where there is no sphere primitive), place things with rigid
  motions, and select edges and faces however the kernel does. A small helper module shared by one
  kernel's implementations is fine. Each case file should still read as an example of that kernel's
  use.
- **It may not:**
  - compute the expected answer;
  - build geometry outside the kernel (no hand-made meshes or surfaces);
  - patch the kernel;
  - catch a crash and report it as a refusal;
  - work around the grading.
- **A swept solid** (everything a solid passes through as it moves, a case in `swept/`) is built
  with the kernel's own sweep of a solid where it has one. Where it has none, the implementation may
  build the same solid from the kernel's other operations when its shape follows from the
  description by elementary geometry, stated in a comment beside the code: a ball's sweep is the
  tube about its center's path with a ball at each end; a box slid along a line is the box at each
  end and each face slid along the line. An envelope the implementation would have to compute
  itself, such as a curve fitted to the points where a moving tool touches its own sweep, is
  geometry made outside the kernel: that case is refused.
- **A workaround is flagged.** An implementation that builds the solid without a feature the
  natural construction calls for, because its kernel lacks it (a sphere primitive, a sweep of a
  solid, a draft, a chamfer with two distances), and makes the same solid from the kernel's other
  operations instead, is flagged outside its code, in `kernels/<kernel>/workarounds.toml`: one
  line per such case, its id and what the kernel lacks, briefly (several joined with commas):

  ```
  "primitives/sphere" = "no sphere primitive"
  "swept/box-slid-aslant" = "no sweep of a solid"
  ```

  The board marks the implementation's result †, and counts each kernel's passes made by
  workaround. A workaround is graded like any other implementation: the flag only says how the
  result was reached. A refusal is never flagged, and nor is composition the case itself describes
  (a hole cut from a block), nor a choice between operations the kernel has.
- **An elegant implementation may be flagged.** One that stands out when the case is read across
  the kernels, as a solution notably cleaner than the others or as one showing an idea in its
  kernel's API or design, may be called so in `kernels/<kernel>/elegance.toml`: one line per such
  case, its id and what makes it elegant, briefly, in the same form as a workaround's:

  ```
  "fillet/box-all-edges" = "every edge rounded in one call, selected as a set"
  ```

  Only a result that passes or builds is flagged, and never a workaround. The board marks the
  implementation's result ✦ and lists every kernel's on its overview and each kernel's on its
  page. Like a workaround's, the flag changes no grade.
- **A case the kernel cannot do** still has a file. The file refuses, saying precisely why (for
  example, "the kernel has no helical curve, and sweeps only by translation and rotation"). That result
  is graded as *refused*.
- **A case with no file is missing.** Nobody has written it yet. That says nothing about the kernel.

## A runner

The harness runs a kernel's `command` from `kernels.json` once per case:

```
<command> CASE OUT.stl [OUT.step]
```

`CASE` is the case's id. A runner whose kernel writes STEP is given `OUT.step` too, and the solid
written there is its whole answer (see [STEP is the answer](#step-is-the-answer) below). Otherwise,
and wherever it has no STEP to give, the runner builds the case with its implementation, writes the
result's triangles to `OUT.stl` (binary or ASCII), and prints one line of JSON as the last line of
its standard output:

```json
{"status": "ok", "volume": 268.082573106329, "area": 402.1238596594935,
 "faces": {"sphere": 1, "cylinder": 1}, "valid": true}
```

or, when the implementation refuses or a kernel operation declines:

```json
{"status": "refused", "reason": "…"}
```

The reply's fields:

- **`volume` and `area`** are the kernel's own measures of its exact solid, not of the triangles.
- **`faces`** counts faces by surface kind: `plane`, `cylinder`, `cone`, `sphere`, `torus`,
  `bspline` or `other`.
- **`valid`** is the kernel's own validity check.

`faces` and `valid` are reported but not graded, because kernels split and check faces differently.
The board marks a solid whose faces far outnumber the other kernels' solids of the case as
*faceted*: a mesh stored as B-rep faces (see [the board](docs/board.md)). The mark is not a grade.
A runner must also answer `--version` with one line naming the kernel and its version, and,
given a case id it has no implementation of, exit without building anything (with an error, or
refusing): the harness times that, as what starting the runner costs.

### STEP is the answer

A B-rep kernel's product is a boundary representation, and STEP is the neutral form of one; a
triangle mesh is not what a kernel is for. So a kernel is graded on its STEP file where it writes
one, and timed on its shortest path to it.

A runner whose kernel can write its solid as STEP (ISO 10303-21, AP203, AP214 or AP242) should,
and says so in `kernels.json` (`"step": true`). A kernel with no STEP writer says that instead, and
why (`"step": "the kernel has no STEP writer"`), so the board can tell a kernel that cannot write
STEP from one whose runner has not been set up to (no `step` at all). A runner given `OUT.step`
builds the case, writes the solid there with its kernel's own STEP writer, as the kernel's users
would (it may not write or rewrite STEP itself), and answers `{"status": "ok"}`: it measures
nothing and writes no triangles. Whatever the runner must do to call the solid built (a check the
kernel itself makes before it hands a solid on) belongs in the run; a measure, a mesh or a validity
check made only for the reply does not. Where the writer declines the solid, the runner writes no
STEP, says why in `step_error`, in the writer's own words where it has them (the kernel
**refused** STEP for that case), and answers as a runner without `OUT.step` does: its measures, its
triangles in `OUT.stl`. A run that writes no STEP and gives no `step_error` **failed** to write
STEP.

The harness reads the file for what the solid is made of (`harness/step.py`): how its boundary is
represented (advanced faces on surfaces, a faceted B-rep, or triangles), its solids, shells, faces,
loops, edges and vertices, its faces by surface and its edges by curve, how many faces are flat
triangles, and how many edges do not bound exactly two sides of faces (none, in a closed shell).
That is the evidence for `faces` that a runner's word alone is not. It judges the file one of:

| verdict | the file holds |
| --- | --- |
| **B-rep** | a solid of advanced faces, each on a surface, its shell closed |
| **faceted** | a mesh stored as a B-rep: a faceted B-rep, or 50 or more advanced faces, at least half of them flat triangles |
| **triangles** | AP242's tessellated faces, and no B-rep |
| **open** | faces on surfaces that do not close into a solid: no solid, or edges that do not pair |
| **empty** | no solid and no faces |

beside **refused** and **failed** where there is no file to judge. A file that is not a closed
solid (open, triangles or empty) is a wrong answer; the verdict itself is shown, and a faceted file
is graded like any other, on its measures.

The harness then measures the file's solid itself (`harness/mass.py`), untimed: its volume and
area, which are graded as a runner's measures are, and its centre of gravity and its moments of
inertia (of unit density, about the centre), which are shown. It reads the exact geometry the file
holds, surface by surface and edge by edge, and integrates over each face by the divergence and
Green theorems, to about 1e-12 of the solid's size; it relies on no kernel's reader, and on none of
the file's curves in a surface's parameters, which writers often leave out. A solid whose faces
face into it (its volume negative) is **inside out**, and a wrong answer; so is a file a loop of
which is not closed (its edges do not meet, vertex to vertex), which is no solid. An edge's curve
may end a writer's tolerance from its vertex: the harness closes gaps of up to 1e-5 of the solid's
size, which moves its measures by about as much. Where the harness cannot
measure a file (a surface or a curve it does not yet read), it says why, and grades the runner's own
measures and triangles in its place, from a second run without `OUT.step`, untimed. Open CASCADE
(`harness/occt.py`, where `cadquery-ocp` is installed) meshes the file for the drawings and the case
page's viewer; nothing it makes is graded.

The file is read as ISO 10303-42 defines it. A face's normal is its surface's normal, turned by its
`same_sense`; each of its loops runs anticlockwise about the face's normal (seen from where the
normal points, the face lies to the left of the loop), turned by its `face_bound`'s `orientation`;
and the faces' normals point out of the solid. Some writers direct a face's loops by its surface's
normal whatever `same_sense` says: read by the standard, such a face is the other side of its loops,
and the solid is not the one meant. Where a file fails so, the harness measures it once more with
the loops taken the writer's way, to say why: the result is still **wrong**, its reason that the
loops run against the standard. Many readers repair both faults silently; a file that needs the
repair is not the solid it says it is.

**Welding.** The harness joins only vertices whose coordinates are exactly equal. A runner may weld
its triangles with its own kernel's tools, but no further: a gap the kernel leaves is the kernel's
to close.

**A kernel with no exact measures.** Some kernels can only measure their meshes. Where it writes
no STEP, a runner for one answers `"measured": "mesh"` in place of `volume` and `area`. The harness then takes both measures
from the triangles it reads and grades them to 1e-3 instead of 1e-6, so the runner must mesh finely
enough for that. Such a kernel is marked *(mesh)* on the board, and `kernels.json` says
`"measured": "mesh"`.

## Grading

Each case and kernel get one outcome. The outcomes, best first:

| outcome | when |
|---|---|
| **pass** | where it wrote STEP: the file holds a closed solid, right side out, and its volume and area, as the harness measures them, are within 1e-6 of the expected values (relative). Where it did not: `volume` and `area` are within 1e-6 of the expected values (1e-3 for a kernel measured from its mesh), **and** the triangles are closed (every edge used exactly twice, once each way) with a volume within 2e-2 of the expected volume |
| **refused** | the runner answered `refused` |
| **wrong** | it answered `ok`, but some condition of a pass fails; the reason is recorded |
| **crashed** | it exited non-zero, or printed no JSON |
| **timeout** | it ran past the time limit (60 s by default) |
| **missing** | the kernel has no implementation of the case; not ranked |

An unsettled case has no answer to meet, so a sound result is graded **built** instead of pass:
the runner answered `ok`, and its STEP file holds a closed solid, right side out, or, with no STEP,
its triangles are closed and consistently oriented. Anything less is **wrong**; refused,
crashed and timeout are as above.

The harness checks the solid itself, from its STEP file or else its triangles: what a kernel's
solid is to every program downstream of it. That keeps a runner's word about its own solid from
being the only evidence.

**What a run costs** is measured by the harness as well, the same way for every kernel, from the
operating system's account of the runner's process: the solid built and written as STEP, and not
meshed or measured, where the kernel writes STEP (see [STEP is the answer](#step-is-the-answer));
reading the file is not counted. None of it is graded:

- its **time**, wall-clock, from starting the runner's process to its exit;
- its **CPU time**, user and system, over every thread of the runner and of the children it
  waited for: more than its time where it kept more than one core busy;
- its **peak memory**, the most the runner held resident at once. The harness starts every
  runner from a process of its own that holds next to nothing, since a process's peak starts at
  that of the process that started it; no run reads under what that process holds;
- with `run.py --one-core`, its **time on one core**: a case the kernel built (passed, built or
  answered wrong) is run a second time pinned to a single core, and only that run's time is
  kept. Its time on one core over its time on all of them is what the machine's other cores
  bought it.

- the size of the kernel's **binary**, on disk, as `binary` in `kernels.json` names its files
  (once a run, not a case);
- what **starting the runner** costs, building nothing: the harness gives the runner a case id
  that no case has, `harness/start`, five times, and keeps the median of their time, CPU time
  and peak memory (and of their time on one core, with `--one-core`).

Every run includes starting the runner, which costs a Python or .NET runtime more time and memory
than a Rust binary; the board's times and CPU times leave that start out, a run's less its
runner's, so that they are the kernel's own (and shows it beside them). They are measures of the machine the run was made on, which a kernel's
results name (its processor, how many cores the run could use, its system, and the least peak
memory a run can read there).

## `kernels.json`

One entry per kernel:

| field | meaning |
|---|---|
| `name` | the kernel's name on the board |
| `kernel` | its home page |
| `license` | shown on the board |
| `backend`, `frontend` | the language the kernel is written in, and the language its implementations are written in: shown on the board |
| `build` | the command that builds the runner (`harness/build.py` runs it) |
| `command` | the runner; `null` lists the kernel as awaiting an implementation |
| `version` | the command that prints the kernel's version |
| `source` | the pattern of an implementation's path |
| `measured` | `"mesh"` for a kernel with no exact measures |
| `step` | `true` for a kernel whose runner writes its solid as STEP when given a third argument; for a kernel that cannot, why not (a string); left out where the runner has not been set up for it |
| `binary` | the files of the kernel's binary, whose size the harness measures after the build: paths or glob patterns relative to the repository (a directory counts every file under it), or a Python distribution, `{"python": "python3", "distribution": NAME}`; beside the language runtime its runner runs on and the system's libraries |
| `note` | shown beside the kernel on the board (e.g. that no new implementations are being written) |
| `reference` | `true` for the kernel whose results the drawings are made from where it passes (else the first kernel that does) |

## Versions

- **1.16:** a kernel that writes STEP is graded on its STEP file and timed on its shortest path
  to it. Given `OUT.step`, the runner builds the solid and writes it as STEP, and measures and
  meshes nothing; the harness measures the file's solid itself, untimed, and grades its volume and
  area to 1e-6, the file a closed solid, right side out (and shows its centre of gravity and its
  inertia). Only where the kernel has no STEP to give (no writer, or its writer declines the
  solid), or the harness cannot measure the file, are its own measures and triangles graded, as in
  1.9. STEP is no longer a run of its own: each case is one run.
- **1.15:** a description may end with its solid stated again as a set of points, under
  `**As a set:**`, in a notation this file defines. It is optional and not graded, and the words
  remain the definition. Grading is as in 1.9.
- **1.14:** every runner whose kernel can write STEP should; a kernel that cannot says why in
  `kernels.json`. A writer's declining is a refusal (`step_error`), told apart from a failure, and
  the harness judges each file (B-rep, faceted, triangles, open or empty), shown and not graded.
  Grading is as in 1.9.
- **1.13:** a runner may write its solid as STEP, with its kernel's own writer, when given a third
  argument; a kernel that does says `"step": true` in `kernels.json`. The harness reads the file
  for what the solid is made of, in an untimed run of its own, and shows it. Grading is as in 1.9.
- **1.12:** a case defines its solid directly, from its geometry and parameters, not as a path
  through other solids that a direct construction of the result would skip; the direct edits,
  which move a solid's faces, are the one exception. Grading is as in 1.9.
- **1.11:** families are removed, and with them a runner's PARAMS argument: twenty runs a case
  cost much CPU time for little coverage the cases they varied did not already give. Grading is as
  in 1.9.
- **1.10:** a case could be a family, stated over parameters and graded at twenty variations drawn
  from a fixed seed, its outcome its worst variation's.
- **1.9:** an implementation may be flagged elegant in its kernel's `elegance.toml`, and the board
  marks and lists it. Grading is unchanged.
- **1.8:** the harness measures each run's CPU time and peak memory beside its time, and, asked,
  its time on one core alone; and what starting each runner costs, which the board's times leave
  out; and the size of each kernel's binary, which `kernels.json` names. A runner given a case id
  it has no implementation of exits without building anything. A kernel's results name the
  machine they were made on. Grading is unchanged.
- **1.7:** a shell's description says what its wall's inside is where moved faces part, cross or
  meet an opening, and states the result exactly, as the solid less its cavity. Grading is
  unchanged.
- **1.6:** where blends meet at a corner, a description says what the corner is (a miter, or a
  corner face) and states the solid as a composition with one answer; a flat corner is asked for
  only where it is one plane. Grading is unchanged.
- **1.5:** an implementation that works around a feature its kernel lacks is flagged in its
  kernel's `workarounds.toml`, and the board marks it. Grading is unchanged.
- **1.4:** a swept solid is built by the kernel's own sweep of a solid, or from its other
  operations where the shape follows by elementary geometry; an envelope the implementation
  computes is refused.
- **1.3:** unsettled cases, with no answer yet, are kept: their sound results are graded *built*,
  and the board shows which kernels agree.
- **1.2:** no integration scripts: an answer is a closed form or two kernels agreeing to 1e-8.
- **1.1:** an answer could come from a numerical integration script beside the case.
- **1:** a case is a description in words with its closed forms. Each kernel implements every case
  in its own idioms, one file per case. A case with no implementation is *missing*.
- **0.2:** seams were never selected edges, and a curve stored in pieces was selected whole. The
  `cylinder-rim` point moved off the seam line. The helix's `x` was defined for any axis. An
  adapter could weld only with its own kernel's tools.
- **0.1:** a kernel with no exact measures could be graded on its own triangles, to 1e-3.
- **0:** cases were JSON programs in a small op language, which one adapter per kernel
  interpreted.

## Changing the suite

- **A new case** is a pull request adding its Markdown file, with its closed forms in `exact`.
  Hard cases are the most valuable, especially ones your own kernel gets wrong. Implementations
  can follow in later pull requests: until then the case is *missing* for every kernel.
- **A new kernel** is a pull request adding `kernels/<kernel>/` and its entry in `kernels.json`. A
  kernel's own authors write its best implementations. Implementations by anyone else are welcome
  too, and the kernel's authors may improve them.
- **A better implementation** of a case is welcome when it is more idiomatic, or when it builds
  what the case describes where the old one refused. Judge it against the description, not against
  the grade.
