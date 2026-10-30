// A solid cylinder of radius 5 and height 10 on the z axis, standing on the xy plane.
// brep-bench solid: cylinder
unit mm
use std

r := 5mm   // radius
h := 10mm  // height

// The base, drawn on the ground (std.top), centred where the z axis meets it.
in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: r)
  radius(r) rim
}
cylinder := solid(face(rim), from: 0mm, to: h)
