// Two cubes of side 10 side by side, from the origin to (10, 10, 10) and from (10, 0, 0) to
// (20, 10, 10), joined into one solid: the face they share disappears.
// brep-bench solid: pair
unit mm
use std

side := 10mm   // each cube's edge

// A rectangle from its corner `corner` to the corner `dx` along and `dy` up from it.
component Rectangle(corner: point, dx: Length, dy: Length) {
  far := point hint((corner.x + dx, corner.y + dy))
  corner distance(dx, along: u) far
  corner distance(dy, along: v) far
  outline := horizontal (south := line(corner)) -> vertical (east := line(p2: far)) ->
             horizontal (north := line(far)) -> vertical (west := line(p2: corner)) -> close
}

// The footprints on the ground (std.top: x and y); the second starts where the first ends.
in std.top {
  o := point
  fix((0, 0)) o
  a := Rectangle(o, dx: side, dy: side)
  b := Rectangle(a.south.p2, dx: side, dy: side)
}
first := solid(a.outline, from: 0mm, to: side)
second := solid(b.outline, from: 0mm, to: side)

pair := solid(first)
second union pair
