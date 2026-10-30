// A torus about the z axis centred on the origin, tube-centre circle radius 10 and tube radius 2,
// with everything where x < 0 cut away.
// brep-bench solid: half
unit mm
use std
use std.place (right_of)

ring := 10mm   // from the axis to the tube's centre
tube := 2mm    // the tube's radius
clear := 1mm   // how far the cutter reaches past the torus

// A rectangle from its corner `corner` to the corner `dx` along and `dy` up from it.
component Rectangle(corner: point, dx: Length, dy: Length) {
  far := point hint((corner.x + dx, corner.y + dy))
  corner distance(dx, along: u) far
  corner distance(dy, along: v) far
  outline := horizontal (south := line(corner)) -> vertical (east := line(p2: far)) ->
             horizontal (north := line(far)) -> vertical (west := line(p2: corner)) -> close
}

// The tube's section in the front plane (x, z), turned about the z axis.
in std.front {
  construction centerline spindle := line(std.origin, hint((0, 1)))
  fix((0, 1)) spindle.p2
  centre := point hint((ring, 0))
  centre right_of(d: ring) std.origin
  section := circle(center: centre) hint(r: tube)
  radius(tube) section
}
half := solid(face(section), about: spindle)

// Where x < 0: a block whose east face is the plane x = 0, reaching past the torus all round.
reach := ring + tube + clear
in std.top {
  o := point
  fix((0, 0)) o
  sw := point hint((-reach, -reach))
  sw distance(reach, along: u) o
  sw distance(reach, along: v) o
  west := Rectangle(sw, dx: reach, dy: 2 * reach)
}
cutter := solid(west.outline, from: -tube - clear, to: tube + clear)
cutter cut half
