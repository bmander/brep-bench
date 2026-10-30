// A solid ball of radius 5 centred on the origin: a half disc turned about its diameter.
// brep-bench solid: ball
unit mm
use std

r := 5mm   // radius

// The meridian in the front plane (x, z): a half disc whose diameter lies on the z axis.
in std.front {
  bottom := point hint((0, -r))
  top := point hint((0, r))
  diameter := vertical line(bottom, top)
  // on the diameter, and the arc's ends both `r` from it: so its middle, said once
  std.origin coincident diameter
  meridian := arc(center: std.origin, start: bottom, end: top) hint(r: r)
  radius(r) meridian
}
ball := solid(face(meridian, diameter), about: diameter)
