// Two balls of radius 5, one centred on the origin and one at (6, 0, 0): their union.
// brep-bench solid: pair
unit mm
use std
use std.place (right_of)

r := 5mm       // each ball's radius
apart := 6mm   // from the first centre to the second, along x

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

// Both meridians in the front plane (x, z), the second centre `apart` along x.
in std.front {
  c2 := point hint((apart, 0))
  c2 right_of(d: apart) std.origin
  first := Ball(std.origin, r: r)
  second := Ball(c2, r: r)
}
pair := solid(first.body)
second.body union pair
