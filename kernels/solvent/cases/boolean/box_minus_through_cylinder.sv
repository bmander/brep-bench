// A cube of side 10 from the origin to (10, 10, 10), drilled straight through along z by a hole
// of radius 3 centred at x = 5, y = 5; the drill runs past both faces, from z = -1 to z = 11.
// brep-bench solid: cube
unit mm
use std

side := 10mm   // the cube's edge
hole_r := 3mm  // the hole's radius
past := 1mm    // how far the drill runs past each face

// The cube's footprint and the hole on the ground (std.top: x and y), the hole at its middle.
in std.top {
  profile := step(u: side) (south := line) -> step(v: side) (east := line) ->
             step(u: 0mm) (north := line) -> step(v: 0mm) (west := line) -> close
  construction diagonal := line(south.p1, east.p2)
  centre := point
  centre midpoint diagonal
  bore := circle(center: centre) hint(r: hole_r)
  radius(hole_r) bore
}
cube := solid(profile, from: 0mm, to: side)
drill := solid(face(bore), from: -past, to: side + past)
drill cut cube
