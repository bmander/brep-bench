// A ball of radius 5 centred on the origin, with a hole of radius 3 bored right through it along
// the z axis.
// brep-bench solid: ring
unit mm
use std

ball_r := 5mm   // the ball's radius
hole_r := 3mm   // the hole's radius

// A ball of radius `r` about `center`: a half disc turned about its diameter, which stands
// upright (along z) through the centre.
component Ball(center: point, r: Length) {
  private bottom := point hint((center.x, center.y - r))
  private top := point hint((center.x, center.y + r))
  private diameter := vertical line(bottom, top)
  // on the diameter, and the arc's ends both `r` from it: so its middle, said once
  center coincident diameter
  private meridian := arc(center: center, start: bottom, end: top) hint(r: r)
  radius(r) meridian
  body := solid(face(meridian, diameter), about: diameter)
}

// The ball's meridian in the front plane (x, z), about the z axis.
in std.front {
  ball := Ball(std.origin, r: ball_r)
}
ring := solid(ball.body)

// The hole, drawn on the ground (std.top) about the z axis, run right through the ball.
in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: hole_r)
  radius(hole_r) rim
}
bore := solid(face(rim), through: ring)
bore cut ring
