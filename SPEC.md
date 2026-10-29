# brep-bench specification, version 0.1

This document defines four things: what a case is, how an adapter is called and must answer, how a
run is graded, and how the suite changes.

## Principles

- **Neutral answers.** Every expected value is a closed form, written in the case beside the number
  (`source`), so it can be checked without trusting any kernel. A case whose answer has no closed
  form takes one from at least two independent kernels that agree, and says so in its source.
- **The same rules for every kernel.** The rules change only by a versioned edit to this file, made
  before any results under the new rules are published.
- **Adapters are separate processes.** A kernel takes part whatever its language or licence, and an
  adapter cannot affect the harness or another kernel.
- **A refusal ranks above a wrong answer.** A kernel that says "I cannot" is more useful than one
  that returns the wrong solid.

## A case

A case is a JSON file `cases/<category>/<name>.json`, written by `cases/generate.py`:

```json
{
  "id": "boolean/box-minus-through-cylinder",
  "tags": ["boolean", "cut", "analytic"],
  "note": "",
  "ops": [
    {"op": "box", "id": "a", "min": [0, 0, 0], "max": [10, 10, 10]},
    {"op": "cylinder", "id": "b", "base": [5, 5, -1], "axis": [0, 0, 1], "radius": 3, "height": 12},
    {"op": "cut", "id": "out", "a": "a", "b": "b"}
  ],
  "result": "out",
  "expect": {
    "volume": {"value": 717.2566611769, "source": "10³ − π·3²·10"},
    "area": {"value": 731.946891451, "source": "600 − 2·9π + 2π·3·10"},
    "valid": true
  }
}
```

The ops run in order. Each `id` names its solid for the ops after it, and `result` names the solid
that is graded. Lengths have no unit. Angles are in degrees. Every axis is a vector, which need not
be unit length.

| op | fields | the solid |
|---|---|---|
| `box` | `min`, `max` | the axis-aligned box between two corners |
| `cylinder` | `base`, `axis`, `radius`, `height` | the cylinder from `base` along `axis` |
| `cone` | `base`, `axis`, `radius1`, `radius2`, `height` | the frustum, with `radius1` at `base` |
| `sphere` | `center`, `radius` | the ball |
| `torus` | `center`, `axis`, `major`, `minor` | the torus about `axis` |
| `extrude` | `polygon` (points), `vector` | the planar polygon swept along `vector` |
| `revolve` | `polygon`, `axis_origin`, `axis`, `angle` | the polygon turned about the line, right-handed |
| `sweep_circle` | `radius`, `helix`, `ends` | the disc square to a helix, swept along it (see below) |
| `union`, `cut`, `common` | `a`, `b` | the Boolean of two solids built before; `cut` is `a` minus `b` |
| `fillet` | `target`, `radius`, `edges` | the target with the selected edges rounded |

**The `helix` field** of `sweep_circle` has these fields:

- `center` and `axis`: the line the helix turns about.
- `radius`: its distance from that line.
- `pitch`: the rise per turn. A pitch of 0 makes the path a circular arc.
- `turns`: how many turns it makes, which may be fractional.

The path starts at `center + radius·x`. Here `x` is a fixed direction square to the axis; for
`axis` = z it is x. The path turns right-handed about `axis`.

**The `ends` field** is `flat` (the end discs) or `round` (a half ball on each end, as a ball rolled
along the path leaves).

**Edge selectors** pick edges by geometry, never by a kernel's own numbering:

- `{"all": true}`: every edge.
- `{"nearest": [p, …]}`: for each point `p`, the edge nearest it.

## An adapter

An adapter is any command, listed in `adapters.json`. The harness runs it as:

```
<command> CASE.json OUT.stl
```

The adapter builds the case's result. It writes the result's triangles to `OUT.stl` (binary or
ASCII) and prints one line of JSON as the last line of its standard output:

```json
{"status": "ok", "volume": 717.2566611769, "area": 731.946891451,
 "faces": {"plane": 6, "cylinder": 1}, "valid": true}
```

or, when the kernel cannot build the case:

```json
{"status": "refused", "reason": "no edge fillet in the public API"}
```

The fields of the reply:

- **`volume` and `area`** are the kernel's own measures of its exact solid, not of the triangles.
- **`faces`** counts faces by surface kind: `plane`, `cylinder`, `cone`, `sphere`, `torus`,
  `bspline` or `other`.
- **`valid`** is the kernel's own validity check.

`faces` and `valid` are reported but not graded, because kernels split faces differently.

**A kernel with no exact measures.** Some kernels can measure only their meshes. An adapter for one
of these answers `"measured": "mesh"` in place of `volume` and `area`. The harness then takes both
measures from the triangles it reads, and grades them to 1e-3 instead of 1e-6, so the adapter must
mesh finely enough for that. Such a kernel is marked *(mesh)* on the board, and `adapters.json`
says so too (`"measured": "mesh"`).

An adapter must also answer `--version` with one line naming the kernel and its version.

## Grading

Each case and kernel get one outcome. The outcomes, best first:

| outcome | when |
|---|---|
| **pass** | `volume` and `area` are within 1e-6 of the expected values (relative; 1e-3 for a kernel measured from its mesh), **and** the triangles are closed (every edge used exactly twice, once each way) with a volume within 2e-2 of the expected volume |
| **refused** | the adapter answered `refused` |
| **wrong** | it answered `ok`, but some condition of a pass fails; the reason is recorded |
| **crashed** | it exited non-zero, or printed no JSON |
| **timeout** | it ran past the time limit (60 s by default) |

The harness checks the triangles itself. They are what a kernel's solid looks like to every program
downstream of it, and checking them keeps an adapter's word about its own solid from being the only
evidence.

Times are reported as wall-clock time, including starting the adapter's process. They are not
graded.

## Versions

- **0.1**: a kernel with no exact measures may be graded on its own triangles, to 1e-3, marked
  *(mesh)* on the board.
- **0**: the first rules.

## Changing the suite

- **A new case** is a pull request to `cases/generate.py`, with its closed form in `source`.
- **A new op** is a pull request to this file, the generator and at least one adapter.
- **A new adapter** is a pull request adding its directory under `adapters/` and its entry in
  `adapters.json`. A kernel's own authors are its best adapter writers. An adapter by anyone else is
  welcome too, and the kernel's authors may fix it.
