// Two cylinders of radius 5 and length 30 crossing at right angles, one along the x axis
// from x = -15 to 15 and one along the y axis from y = -15 to 15: their union.
// brep-bench solid: cross
unit mm
use std

r := 5mm          // each cylinder's radius
half_len := 15mm  // from the origin to each cylinder's end

// Each cylinder's end circle is drawn in the plane square to its axis through the origin, and
// swept along that plane's normal both ways: std.side (y, z) faces along x, std.front (x, z)
// along y.
in std.side {
  o_x := point
  fix((0, 0)) o_x
  rim_x := circle(center: o_x) hint(r: r)
  radius(r) rim_x
}
in std.front {
  rim_y := circle(center: std.origin) hint(r: r)
  radius(r) rim_y
}
along_x := solid(face(rim_x), from: -half_len, to: half_len)
along_y := solid(face(rim_y), from: -half_len, to: half_len)

cross := solid(along_x)
along_y union cross
