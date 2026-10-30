// A cube of side 10 with all twelve of its edges rounded to radius 1; three rounds meet at each
// corner.
// brep-bench solid: cube
unit mm
use std
use std.rounds (round)

side := 10mm   // the cube's edge
r := 1mm       // every round's radius

// The footprint on the ground (std.top), from the origin to (10, 10).
in std.top {
  profile := step(u: side) (south := line) -> step(v: side) (east := line) ->
             step(u: 0mm) (north := line) -> step(v: 0mm) (west := line) -> close
}
cube := solid(profile, from: 0mm, to: side)

round(r: r) cube
