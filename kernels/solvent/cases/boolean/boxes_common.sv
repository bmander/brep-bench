// Two blocks 10 high standing on the xy plane, one over x and y from 0 to 10 and one from 5 to
// 15: the part common to both.
// brep-bench solid: common_part
unit mm
use std

side := 10mm     // each block's footprint is a square this wide
height := 10mm
shift := 5mm     // the second block's corner, along x and along y

// A rectangle from its corner `corner` to the corner `dx` along and `dy` up from it.
component Rectangle(corner: point, dx: Length, dy: Length) {
  far := point hint((corner.x + dx, corner.y + dy))
  corner distance(dx, along: u) far
  corner distance(dy, along: v) far
  outline := horizontal (south := line(corner)) -> vertical (east := line(p2: far)) ->
             horizontal (north := line(far)) -> vertical (west := line(p2: corner)) -> close
}

// The footprints on the ground (std.top: x and y), the first at the origin.
in std.top {
  o := point
  fix((0, 0)) o
  q := point
  fix((shift, shift)) q
  a := Rectangle(o, dx: side, dy: side)
  b := Rectangle(q, dx: side, dy: side)
}
first := solid(a.outline, from: 0mm, to: height)
second := solid(b.outline, from: 0mm, to: height)

common_part := solid(first)
second common common_part
