// A quarter of a washer: the rectangle x from 6 to 10, z from 0 to 4 in the xz plane, revolved
// 90 degrees about the z axis, from +x towards +y.
// brep-bench solid: sector
unit mm
use std
use std.place (right_of)

ri := 6mm   // inner radius
ro := 10mm  // outer radius
h := 4mm    // height

in std.front {
  construction centerline spindle := line(std.origin, hint((0, 1)))
  fix((0, 1)) spindle.p2
  inner := point hint((ri, 0))
  inner right_of(d: ri) std.origin
  section := step(u: ro) (bottom := line(inner)) -> step(dv: h) (outside := line) ->
             step(u: ri) (top := line) -> (inside := line(p2: inner)) -> close
}
sector := solid(section, about: spindle, sweep: 90deg)
