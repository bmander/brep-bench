# Adversarial cases against the frontrunners

In November 2026 three kernels stood at the top of the board: cadrs (513 of 538 cases passed),
next.BREP (512) and springrep (487). The cases tagged `"adversarial"` were written to test whether
those passes come from robust geometry or from code shaped around the cases. Each one:

- perturbs a case the kernels pass, by a change of size, a rigid turn, a flatter or rounder
  quadric, or one more concave edge;
- or sits just outside a lane in a kernel's source that names a brep-bench case.

Every case was run on all three kernels, at the pinned versions the board runs: cadrs `dc4e99c`,
BREP_kernel 0.28.0 and springrep 2.1.0. Each kernel's implementation copies its own implementation
of the nearest existing case.

## Result

43 cases. Each cell is the harness's verdict on the kernel's own STEP file.

| kernel | pass | wrong | refused |
|---|---|---|---|
| cadrs | 19 | 15 | 9 |
| next.BREP | 18 | 16 | 9 |
| springrep | 26 | 6 | 11 |

- 33 cases get a wrong answer from at least one of the three.
- Five cases get a wrong answer or a refusal from all three: the two spheroid shells, the box with
  a spheroidal cavity grown and shrunk, and the flat elliptic cylinder grown.
- No kernel is wrong on a case that the other two refuse. In every row, at least one kernel either
  matches the closed form to round-off or the closed form has a second, independent check:
  - OCCT agreeing to 1e-10;
  - high-precision quadrature, the offset meridian revolved in mpmath to 25 digits;
  - or, for a scaled or turned case without a closed form, the settled answer carried over exactly
    (×s³ and ×s², or unchanged).

| case | cadrs | next.BREP | springrep |
|---|---|---|---|
| `fillet/half-cylinder-every-edge-rounded` | pass | pass | refused |
| `loft/square-to-round-three-times-larger` | refused | pass | pass |
| `offset/barrel-shrunk` | pass | refused | pass |
| `offset/blind-bored-block-grown-past-its-bore` | pass | pass | **wrong** (-2.6e-03 V) |
| `offset/box-with-a-spheroidal-cavity-grown` | refused | **wrong** (+1.9e-02 V) | refused |
| `offset/box-with-a-spheroidal-cavity-shrunk` | refused | **wrong** (+1.7e-01 V) | refused |
| `offset/flat-elliptic-cylinder-grown` | refused | refused | refused |
| `offset/l-block-rounded-inside-grown-past-its-round` | pass | **wrong** (+3.4e-04 V) | **wrong** (-2.1e-04 V) |
| `offset/oblate-spheroid-grown` | refused | pass | refused |
| `offset/plate-with-a-square-hole-grown` | pass | **wrong** (+1.5e-04 V) | pass |
| `offset/plus-shaped-plate-grown` | pass | **wrong** (+2.6e-04 V) | pass |
| `offset/plus-shaped-plate-shrunk` | pass | refused | pass |
| `offset/pocketed-block-grown-past-its-corners` | pass | **wrong** (+7.3e-05 V) | **wrong** (-2.2e-03 V) |
| `offset/recessed-plate-grown-past-its-recess` | pass | refused | **wrong** (-3.3e-03 V) |
| `offset/square-pocketed-block-grown` | pass | **wrong** (+2.2e-05 V) | pass |
| `offset/star-prism-grown` | pass | **wrong** (+9.3e-06 V) | pass |
| `offset/torus-grown-to-its-hole` | **wrong** (+2.5e-01 V) | refused | pass |
| `offset/u-shaped-plate-grown` | pass | **wrong** (+9.5e-05 V) | pass |
| `offset/wide-hole-plate-grown-by-its-hole` | **wrong** (+1.3e-02 V) | **wrong** (+1.3e-02 V) | pass |
| `offset/windowed-plate-grown-past-its-corners` | pass | **wrong** (+2.5e-04 V) | **wrong** (-5.5e-05 V) |
| `scale/ball-screwed-round-a-tight-helix-ten-times-larger` | **wrong** (-1.7e-04 V) | pass | refused |
| `scale/box-screwed-inside-a-rod-a-hundred-times-larger` | **wrong** (+7.5e-05 V) | refused | pass |
| `scale/cone-spun-about-a-line-through-it-ten-times-larger` | **wrong** (-4.9e-04 V) | refused | pass |
| `scale/drilled-plate-grown-by-its-hole-ten-times-larger` | **wrong** (+2.6e-03 V) | pass | pass |
| `scale/micro-cylinder-rim-rounded-unequally` | **wrong** (+2.5e-02 V) | pass | refused |
| `scale/spline-bar-screwed-through-rod-ten-times-larger` | **wrong** (-2.0e-03 V) | pass | pass |
| `scale/villarceau-half-torus-ten-thousand-times-larger` | **wrong** (+1.0e+00 V) | refused | pass |
| `scale/viviani-common-a-thousand-times-larger` | refused | pass | pass |
| `scale/viviani-cut-a-thousand-times-larger` | **wrong** (+4.0e-01 V) | pass | pass |
| `scale/viviani-union-a-thousand-times-larger` | **wrong** (+2.5e-01 V) | pass | pass |
| `shell/bored-frustum-sealed-below-its-opening-at-three-point-one-times-the-size` | **wrong** (-2.4e-02 V) | pass | pass |
| `shell/oblate-spheroid-hollowed` | refused | **wrong** (+8.0e-01 V) | refused |
| `shell/prolate-spheroid-hollowed` | refused | **wrong** (+5.5e-01 V) | refused |
| `singular/ball-dimpled-a-hair-from-its-cutters-pole` | pass | pass | refused |
| `singular/sectors-sharing-an-apex-ten-times-larger` | refused | pass | pass |
| `sweep/circle-along-a-nearly-full-arc` | pass | **wrong** (-3.8e-06 V) | pass |
| `sweep/helical-pipe-forty-turns` | pass | **wrong** (-1.0e-05 V) | pass |
| `sweep/square-along-a-nearly-full-arc` | pass | **wrong** (-3.8e-06 V) | pass |
| `swept/ball-screwed-round-a-tight-helix-on-a-tilted-axis` | **wrong** (-1.6e-04 V) | pass | refused |
| `swept/box-screwed-inside-a-tilted-rod` | **wrong** (+1.2e-04 V) | pass | pass |
| `swept/box-slid-along-its-face` | pass | refused | pass |
| `swept/torus-screwed-through-a-tilted-rod` | **wrong** (-3.4e-03 V) | pass | pass |
| `thicken/v-bracket-sharp` | pass | pass | **wrong** (-1.9e-01 V) |

## Why they fail

### cadrs: absolute tolerances and a chain of shape lanes

- **Scale.** Turning a case off its axes or moving it far from the origin found nothing: about 40
  variants passed to 1e-15. Scaling the same cases broke them. The tolerances in
  `native/tol.rs` are absolute millimetres (CONFUSION 1e-7, POINT 1e-6, LOOP_GAP 1e-5), and many
  inline literals are too. Examples:
  - The Viviani cut, union and common ×1000, and Villarceau's half torus ×10⁴, return the whole
    ball or torus. The cut and the union are wrong by +40% and +100%, and the common is refused.
  - The unequal round on the cylinder's rim, ×0.01, is silently left off.
  - The square-to-round loft ×3 (a 120 mm part) refuses: "The surfaces don't close".
- **Sweeps of a solid.** A solid's sweep tries shape-specific lanes in turn and falls back to a
  mesh when none fits (`native/sweep_solid.rs:3398-3484`). The lanes include:
  - `whole_turn_of_round`;
  - `slid_ellipsoid`;
  - `ring_moved_flat`;
  - `polyhedron_turned`;
  - a "pipe slid" lane keyed on the session remembering how the body was made
    (`k.pipes.get(&body.0)`);
  - `screwed_ball`, whose pieces are sewn with an absolute 1e-5.

  Tilt the screw axis of a passing case (the tight-helix ball, the box in a rod, the torus in a
  rod) or scale it by 10, and every lane declines. The result is a faceted solid, graded wrong.
  `native/sweep_faces.rs:233` also gives a sweep by faces a 4-second wall-clock budget, so
  exactness can depend on the machine.
- **Offsets.**
  - A plate grown by a hole radius of 20 or more loses its dimples. cadrs and next.BREP both
    return the grown box, 366933.317256, and they agree to every digit. Under SPEC.md's rule for
    unsettled cases (two kernels agreeing to 1e-8), that would have settled the wrong answer.
  - A torus grown to its hole writes two solids.
  - Smooth offsets of a spheroid refuse.
- **Runner.** `cases/tangent/thin_lens.rs` and `cases/tangent/hairline_slot.rs` tighten the
  session's tolerances 100× for those cases alone. It is public API, but it tunes the kernel to
  the case.

### next.BREP: offset rounds, station counts and a STEP writer

The kernel's lanes named after brep-bench cases are mostly general. These survived turns, moves
and changes of count:

- `box_cavity.rs`, `sphere_cut_shell.rs`, `revolved_*`, `prism_*`, `pipe_parallel.rs`,
  `outline_offset.rs`;
- the soundness short-cuts in `disjoint_balls.rs` and `concentric_balls.rs`.

Its 16 wrong answers come from four defects:

- **The rounded growth of a prism with concave vertical edges.** It is wrong by 1e-5 to 3e-4 on a
  plus, a U, a star, a square hole, a pocket, a window and a rounded L; `offset/l-block-grown`
  passes.
- **Path sweeps with a station count tuned to the cases.** `sweep_topology/sweep.rs`:
  - keeps 32 stations for any path turning up to 360°, for "golden parity", so a sweep along an
    arc of 350° is wrong by 4e-6;
  - takes 64 stations per helix turn, chosen from `sweep/helical-pipe`'s error and clamped at
    1024, so the pipe of 40 turns is wrong by 1e-5.
- **The STEP writer.** It loses an inward-facing closed B-spline face from an offset. The
  spheroid shells are written as solid spheroids (+80%, +55%), and the box with a spheroidal
  cavity loses its cavity. Its own B-rep measures right.
- **Narrow preconditions in two lanes.**
  - The torus lane takes only s > R(1 + 1e-9) (`torus_body.rs:67`), so a torus grown exactly to
    its hole refuses.
  - The prism-growth lane's fit (`prism_growth.rs:44-45,192`: 64 to 512 stations against a 1e-9
    bar) declines a flatter ellipse.

### springrep: an offset dimple and a thicken built for right angles

- **Offset dimples.** `edit.rs:253-369` takes any concave cylinder face of radius below the
  distance to be a perpendicular through hole. It then cuts a revolved dimple about every circle
  edge beside a plane, including a pocket's floor arcs and inside-corner rounds. The result:
  - a grown blind bore gets a phantom toroidal void, as a second shell;
  - a grown recess, pocket, window and rounded L lose or gain material, by 5e-5 to 3e-3.
- **Sharp thickening.** `sheet/thicken.rs:86-101` (Sharp) thickens the rounded result in place,
  which is correct only for creases of 90° or more. A 60° bracket comes back as two disjoint
  solids, 19% short, and the kernel reports them `valid`. Every crease from 30° to 89° is wrong;
  90° to 150° is exact.
- **Refusals.**
  - Rounds meeting at a vertex are refused unless every face is a plane.
  - A boolean whose meeting circle passes 9e-5 from a sphere's pole refuses ("a region's
    boundary does not close"); exactly through the pole, it passes.
