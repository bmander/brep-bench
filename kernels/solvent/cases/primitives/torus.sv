// A solid torus about the z axis, centred on the origin: tube-centre circle radius 10, tube
// radius 2.
// brep-bench solid: torus
unit mm
use std
use std.place (right_of)

ring := 10mm   // from the axis to the tube's centre
tube := 2mm    // the tube's radius

// The tube's section in the front plane (x, z), turned about the z axis.
in std.front {
  construction centerline spindle := line(std.origin, hint((0, 1)))
  fix((0, 1)) spindle.p2
  centre := point hint((ring, 0))
  centre right_of(d: ring) std.origin
  section := circle(center: centre) hint(r: tube)
  radius(tube) section
}
torus := solid(face(section), about: spindle)
