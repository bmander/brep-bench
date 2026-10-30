// A circle of radius 2 swept along a quarter turn of an arc of radius 10 in the xy plane about the
// z axis, from (10, 0, 0) to (0, 10, 0); the section stays square to the path, the ends are flat.
// brep-bench solid: pipe
unit mm
use std
use std.stations

path_r := 10mm    // the arc's radius
wire_r := 2mm     // the section's radius
bend := 90deg     // how far round the arc runs

// The section at station `s`: a circle in the plane square to the guide there, centred on it.
component Section(g: arc, up: axis, r: Length, s: Scalar) {
  st := std.stations.OnArc(g, up: up, s: s)
  in st.n {
    rim := circle(center: st.n.origin) hint(r: r)
    radius(r) rim
  }
  f := face(rim)
}

// The guide, drawn in plan (std.top): about the z axis, from the x axis a quarter turn round.
in std.top {
  o := point
  fix((0, 0)) o
  entry := point hint((path_r, 0))
  exit := point hint((0, path_r))
  construction centerline start_radius := horizontal line(o, entry)
  construction centerline end_radius := line(o, exit)
  start_radius angle(bend) end_radius
  construction centerline guide := arc(center: o, start: entry, end: exit) hint(r: path_r)
  radius(path_r) guide
}
param s: Scalar over (0, 1)
pipe := sweep(Section(guide, std.z, r: wire_r).f over s)
