// A cube of side 10 from the origin to (10, 10, 10), the edge where its faces x = 10 and y = 10
// meet rounded to radius 2.
// brep-bench solid: cube
unit mm
use std
use std.rounds (round)

side := 10mm   // the cube's edge
r := 2mm       // the round's radius

// The footprint on the ground (std.top), walked from the origin: `east` is the side at x = 10,
// `north` the side at y = 10, and each names the face it sweeps.
in std.top {
  profile := step(u: side) (south := line) -> step(v: side) (east := line) ->
             step(u: 0mm) (north := line) -> step(v: 0mm) (west := line) -> close
}
cube := solid(profile, from: 0mm, to: side)

cube.east round(r: r) cube.north
