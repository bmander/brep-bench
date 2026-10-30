// An L-shaped plate: the outline (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15) in the xy
// plane, extruded 8 along +z.
// brep-bench solid: plate
unit mm
use std

long_leg := 20mm   // along x
short_leg := 15mm  // along y
thick := 5mm       // each leg's width
depth := 8mm       // the extrusion

// The outline on the ground (std.top), walked from the origin, one number a run.
in std.top {
  outline := step(u: long_leg) (a := line) -> step(v: thick) (b := line) ->
             step(u: thick) (c := line) -> step(v: short_leg) (d := line) ->
             step(u: 0mm) (e := line) -> step(v: 0mm) (f := line) -> close
}
plate := solid(outline, from: 0mm, to: depth)
