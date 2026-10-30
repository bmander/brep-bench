// A plate from the origin to (100, 20, 10), drilled nine times through along z with holes of
// radius 2 at y = 10 and x = 10, 20, ... 90, each hole its own cut.
// brep-bench solid: plate
unit mm
use std

length := 100mm
width := 20mm
thick := 10mm
hole_r := 2mm
pitch := 10mm   // from the plate's end to the first hole, and from each hole to the next
holes := 9

// The plate's outline on the ground (std.top: x and y), and its centre line, on which the
// holes are drilled.
in std.top {
  outline := step(u: length) (south := line) -> step(v: width) (east := line) ->
             step(u: 0mm) (north := line) -> step(v: 0mm) (west := line) -> close
  construction centreline := line(hint((0, width / 2)), hint((length, width / 2)))
  centreline.p1 midpoint west
  centreline.p2 midpoint east
}
plate := solid(outline, from: 0mm, to: thick)

// One drill after another, each a cut of its own, `pitch` along the centre line from the one
// before and run past both faces.
repeat holes as i {
  c := point hint(((i + 1) * pitch, width / 2)) in std.top
  c coincident centreline
  centreline.p1 distance((i + 1) * pitch, along: u) c
  rim := circle(center: c) hint(r: hole_r) in std.top
  radius(hole_r) rim
  drill := solid(face(rim), from: -1mm, to: thick + 1mm)
  drill cut plate
}
