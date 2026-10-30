// A truncated cone on the z axis from z = 0 to z = 6, radius 5 at its base and 2 at its top.
// brep-bench solid: frustum
unit mm
use std

r_base := 5mm   // radius at z = 0
r_top := 2mm    // radius at z = 6
h := 6mm        // height

// The half-section in the front plane (x, z): out along the base, up the slant to the top's
// radius, back to the axis, and down the axis — every number a radius or a height.
in std.front {
  profile := step(u: r_base) (base := line) -> step(u: r_top, v: h) (slant := line) ->
             step(u: 0mm) (top := line) -> step(v: 0mm) (spine := line) -> close
}
frustum := solid(profile, about: spine)
