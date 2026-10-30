// A pipe of circular section, radius 1, following a helix about the z axis of radius 10 rising 5
// per turn: from (10, 0, 0), right-handed (from +x towards +y), three whole turns, the section
// square to the path. Each end is closed by a half ball of radius 1, as a ball rolled along the
// helix would leave.
// brep-bench solid: pipe
unit mm
use std
use std.place (right_of)
use std.motion (screws)

coil_r := 10mm   // the helix's radius
wire_r := 1mm    // the pipe's radius
pitch := 5mm     // how far it rises each turn
turns := 3

// A ball of radius `r` about `center`: a half disc turned about its upright diameter.
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

// The ball starts on the x axis, `coil_r` out, drawn in the front plane (x, z).
in std.front {
  construction centerline spindle := line(std.origin, hint((0, 1)))
  fix((0, 1)) spindle.p2
  start := point hint((coil_r, 0))
  start right_of(d: coil_r) std.origin
  ball := Ball(start, r: wire_r)
}

// The ball rolled along the helix: a screw about the z axis climbing `pitch` a turn, and the
// pipe everything the ball passes through over three turns.
climb := frame
climb screws(pitch) spindle
pipe := sweep(ball.body over t in (0deg, turns * 360deg), under: climb)
