# brep-bench specification, version 1.6

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
  agree to 1e-8 on both measures (the value is their mean). Every result is also checked by its
  own triangles: watertight, consistently oriented, bounding the expected volume.

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

A ball of radius 5 centred on the origin, with a hole of radius 3 bored right through it along the
z axis.
```

`tex` is the same closed form in LaTeX, which the case's page typesets; it is left out where there
is no closed form. It is a TOML literal string (in single quotes), so its backslashes stay as written.

The header may also say where a case came from: `origin = "geomsolver rust/examples/solid_flange.sv"`.

How a description is written:

- **Coordinates** are right-handed x, y, z, with z up. Lengths have no unit. Angles are in degrees,
  and a turn about an axis is right-handed unless the text says otherwise.
- **It names the solid and where it lies.** Placement is not graded (volume and area do not depend
  on it), but it keeps every kernel's result in the same place, so the drawings and the 3D viewer
  can compare them.
- **It says what to build, not how.** Hints that keep a construction away from an accidental
  degeneracy are allowed ("if you cut the hole with a cylinder, make it run past both faces"). So is
  saying which operation a case is about ("drill them one after another, each hole its own cut").
- **It must be unambiguous enough to have one answer.** If two careful readers could build
  different solids from it, that is a bug in the case.
- **Where blends meet at a corner, it says what the corner is, as a composition.** Chamfers or
  fillets meeting at a vertex can close it more than one way: running on until they meet in a
  mitre, or cut off by a corner face of their own. The words of one kernel's blend are no
  definition, so the description says which, and states the solid as a composition of operations
  that have one answer: the solid cut by the chamfers' planes, and by each corner's plane where the
  corner is cut flat (`chamfer/cube-all-edges`); and where the case is about a mitre, the edges
  chamfered one after another, each its own chamfer ending on the faces at its ends
  (`chamfer/cube-all-edges-mitred`). A corner is
  described only where that composition is one solid. A flat corner face through the corners the
  faces are left with nearest a vertex is one plane where three faces meet, but where four or more
  do only when those corners happen to lie in a plane (the octahedron's, by its symmetry, and not
  a pyramid's apex on a rectangle), so no case asks for a flat corner where they do not.

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
  sphere as a revolved half disc, where there is no sphere primitive), place things with rigid
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
  tube about its centre's path with a ball at each end; a box slid along a line is the box at each
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
- **A case the kernel cannot do** still has a file. The file refuses, saying precisely why (for
  example, "truck has no helical curve, and sweeps only by translation and rotation"). That result
  is graded as *refused*.
- **A case with no file is missing.** Nobody has written it yet. That says nothing about the kernel.

## A runner

The harness runs a kernel's `command` from `kernels.json` once per case:

```
<command> CASE OUT.stl
```

`CASE` is the case's id. The runner builds that case with its implementation, writes the result's
triangles to `OUT.stl` (binary or ASCII), and prints one line of JSON as the last line of its
standard output:

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
A runner must also answer `--version` with one line naming the kernel and its version.

**Welding.** The harness joins only vertices whose coordinates are exactly equal. A runner may weld
its triangles with its own kernel's tools, but no further: a gap the kernel leaves is the kernel's
to close.

**A kernel with no exact measures.** Some kernels can only measure their meshes. A runner for one
answers `"measured": "mesh"` in place of `volume` and `area`. The harness then takes both measures
from the triangles it reads and grades them to 1e-3 instead of 1e-6, so the runner must mesh finely
enough for that. Such a kernel is marked *(mesh)* on the board, and `kernels.json` says
`"measured": "mesh"`.

## Grading

Each case and kernel get one outcome. The outcomes, best first:

| outcome | when |
|---|---|
| **pass** | `volume` and `area` are within 1e-6 of the expected values (relative; 1e-3 for a kernel measured from its mesh), **and** the triangles are closed (every edge used exactly twice, once each way) with a volume within 2e-2 of the expected volume |
| **refused** | the runner answered `refused` |
| **wrong** | it answered `ok`, but some condition of a pass fails; the reason is recorded |
| **crashed** | it exited non-zero, or printed no JSON |
| **timeout** | it ran past the time limit (60 s by default) |
| **missing** | the kernel has no implementation of the case; not ranked |

An unsettled case has no answer to meet, so a sound result is graded **built** instead of pass:
the runner answered `ok`, and its triangles are closed and consistently oriented. Triangles that are
not closed are **wrong**; refused, crashed and timeout are as above.

The harness checks the triangles itself. They are what a kernel's solid looks like to every program
downstream of it, and checking them keeps a runner's word about its own solid from being the only
evidence.

Times are reported as wall-clock time, including starting the runner's process. They are not
graded.

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
| `note` | shown beside the kernel on the board (e.g. that no new implementations are being written) |
| `reference` | `true` for the kernel whose results the drawings are made from where it passes (else the first kernel that does) |

## Versions

- **1.6:** where blends meet at a corner, a description says what the corner is (a mitre, or a
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
