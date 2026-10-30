// A cylinder of radius 5 on the z axis from z = 0 to 10, with a cylinder of radius 3 on the
// same axis cut from it that runs exactly from z = 0 to z = 10: its ends lie in the first's.
// brep-bench solid: tube
unit mm
use std

r_out := 5mm   // the cylinder's radius
r_in := 3mm    // the hole's radius
h := 10mm      // the cylinder's length

// Both circles are drawn on the ground (std.top), about the z axis.
in std.top {
  o := point
  fix((0, 0)) o
  outside := circle(center: o) hint(r: r_out)
  radius(r_out) outside
  inside := circle(center: o) hint(r: r_in)
  radius(r_in) inside
}
tube := solid(face(outside), from: 0mm, to: h)

// The core, running exactly as far as the cylinder: its ends lie in the cylinder's ends.
core := solid(face(inside), from: 0mm, to: h)
core cut tube
