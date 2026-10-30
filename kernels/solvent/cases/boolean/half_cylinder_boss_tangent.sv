// A cube of side 10 from the origin to (10, 10, 10), united with a cylinder of radius 5 and
// height 10 standing on the xy plane on the axis x = 10, y = 5: half of it inside the cube, its
// wall touching the faces y = 0 and y = 10 along the cube's edges at x = 10.
// brep-bench solid: block
unit mm
use std

side := 10mm   // the cube's edge, and the cylinder's height
boss_r := 5mm  // the cylinder's radius

// The cube's footprint and the cylinder's base on the ground (std.top: x and y). The base is
// centred on the cube's east side; being as wide as the cube, it passes through both of that
// side's ends, which is claimed rather than stated.
in std.top {
  profile := step(u: side) (south := line) -> step(v: side) (east := line) ->
             step(u: 0mm) (north := line) -> step(v: 0mm) (west := line) -> close
  axis_foot := point
  axis_foot midpoint east
  boss_rim := circle(center: axis_foot) hint(r: boss_r)
  radius(boss_r) boss_rim
  claim east.p1 coincident boss_rim
  claim east.p2 coincident boss_rim
}
cube := solid(profile, from: 0mm, to: side)
boss := solid(face(boss_rim), from: 0mm, to: side)

block := solid(cube)
boss union block
