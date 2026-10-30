// A stepped shaft on the z axis, made one solid: radius 5 from z = 0 to 10, and radius 3 from
// z = 10 to 20 standing on top of it.
// brep-bench solid: shaft
unit mm
use std

r_lower := 5mm   // the lower cylinder's radius
r_upper := 3mm   // the upper cylinder's radius
step_h := 10mm   // each cylinder's length

// Both cylinders' ends are drawn on the ground (std.top), about the z axis.
in std.top {
  o := point
  fix((0, 0)) o
  lower_rim := circle(center: o) hint(r: r_lower)
  radius(r_lower) lower_rim
  upper_rim := circle(center: o) hint(r: r_upper)
  radius(r_upper) upper_rim
}
lower := solid(face(lower_rim), from: 0mm, to: step_h)
upper := solid(face(upper_rim), from: step_h, to: 2 * step_h)

shaft := solid(lower)
upper union shaft
