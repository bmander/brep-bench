// Two cubes of side 10 joined into one solid: the first from the origin to (10, 10, 10), the
// second from (5, 0, 10) to (15, 10, 20), standing on the first and covering half its top.
// brep-bench solid: stack
unit mm
use std

side := 10mm       // each cube's edge
overhang := 5mm    // how far along x the upper cube is set over

// A rectangle from its corner `corner` to the corner `dx` along and `dy` up from it.
component Rectangle(corner: point, dx: Length, dy: Length) {
  far := point hint((corner.x + dx, corner.y + dy))
  corner distance(dx, along: u) far
  corner distance(dy, along: v) far
  outline := horizontal (south := line(corner)) -> vertical (east := line(p2: far)) ->
             horizontal (north := line(far)) -> vertical (west := line(p2: corner)) -> close
}

// The footprints on the ground (std.top: x and y); the upper one is set over along x.
in std.top {
  o := point
  fix((0, 0)) o
  q := point
  fix((overhang, 0)) q
  a := Rectangle(o, dx: side, dy: side)
  b := Rectangle(q, dx: side, dy: side)
}
lower := solid(a.outline, from: 0mm, to: side)
upper := solid(b.outline, from: side, to: 2 * side)

stack := solid(lower)
upper union stack
