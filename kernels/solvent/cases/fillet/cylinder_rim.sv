// A cylinder of radius 5 on the z axis from z = 0 to z = 10, the circular edge around its top
// face rounded to radius 1.
// brep-bench solid: cylinder
unit mm
use std
use std.rounds (round)

cyl_r := 5mm    // the cylinder's radius
h := 10mm       // its height
r := 1mm        // the rim's round

// The base on the ground (std.top), about the z axis. Swept up, the cap at the higher extent is
// `near`: the top face.
in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: cyl_r)
  radius(cyl_r) rim
}
cylinder := solid(face(rim), from: 0mm, to: h)

cylinder.near round(r: r) cylinder
