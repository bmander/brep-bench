// A ball of radius 5 centred on the origin, with everything below the xy plane cut away.
// brep-bench solid: dome
unit mm
use std

r := 5mm   // the ball's radius

// A square centred on `center`, `half` from it to each side.
component Square(center: point, half: Length) {
  sw := point hint((center.x - half, center.y - half))
  ne := point hint((center.x + half, center.y + half))
  sw distance(half, along: u) center
  sw distance(half, along: v) center
  center distance(half, along: u) ne
  center distance(half, along: v) ne
  outline := horizontal (south := line(sw)) -> vertical (east := line(p2: ne)) ->
             horizontal (north := line(ne)) -> vertical (west := line(p2: sw)) -> close
}

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
  ball := Ball(std.origin, r: r)
}
dome := solid(ball.body)

// Below the ground: a block under std.top wider and deeper than the ball, cut away.
in std.top {
  o := point
  fix((0, 0)) o
  foot := Square(o, half: r + 1mm)
}
below := solid(foot.outline, from: -r - 1mm, to: 0mm)
below cut dome
