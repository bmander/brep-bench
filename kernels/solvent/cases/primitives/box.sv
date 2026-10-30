// A rectangular block 10 long in x, 20 in y and 30 in z, from the origin to (10, 20, 30).
// brep-bench solid: box
unit mm
use std

length_x := 10mm
width_y := 20mm
height_z := 30mm

// The footprint on the ground (std.top: u along x, v along y), its corner at the origin.
in std.top {
  profile := step(u: length_x) (south := line) -> step(v: width_y) (east := line) ->
             step(u: 0mm) (north := line) -> step(v: 0mm) (west := line) -> close
}
box := solid(profile, from: 0mm, to: height_z)
