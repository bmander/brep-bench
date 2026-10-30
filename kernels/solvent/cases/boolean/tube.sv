// A cylinder of radius 5 on the z axis from z = 0 to 10, with a hole of radius 3 bored along
// its axis; the drill runs past both ends, from z = -1 to z = 11.
// brep-bench solid: tube
unit mm
use std

r_out := 5mm   // the cylinder's radius
r_in := 3mm    // the hole's radius
h := 10mm      // the cylinder's length
past := 1mm    // how far the drill runs past each end

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

// The hole, run past both ends.
core := solid(face(inside), from: -past, to: h + past)
core cut tube
