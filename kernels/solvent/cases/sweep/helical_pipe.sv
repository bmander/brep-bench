// A pipe of circular section, radius 1, following a helix about the z axis of radius 10 rising 5
// per turn: from (10, 0, 0), right-handed (from +x towards +y), three whole turns. The section
// stays square to the path, and the ends are flat.
// brep-bench solid: pipe
unit mm
use std
use std.stations

coil_r := 10mm   // the helix's radius
wire_r := 1mm    // the section's radius
pitch := 5mm     // how far it rises each turn
turns := 3

// The section at station `s`: a circle in the plane square to the helix there, centred on it.
// `OnHelix` carries std.top by the screw about its normal (the z axis), starting on the x axis.
component Coil(r: Length, wire: Length, pitch: Length, turns: Scalar, s: Scalar) {
  st := std.stations.OnHelix(std.top, r: r, pitch: pitch, turns: turns, s: s)
  in st.n {
    rim := circle(center: st.n.origin) hint(r: wire)
    radius(wire) rim
  }
  f := face(rim)
}

param s: Scalar over (0, 1)
pipe := sweep(Coil(r: coil_r, wire: wire_r, pitch: pitch, turns: turns).f over s)
