+++
title = "A square-to-round transition"
tags = ["loft", "ruled", "transition", "unequal-sections", "hull", "cone"]

[volume]
value = 35068.58347057703
exact = "28000 + 2250π = 30(1600/3 + 160·15/6 + 225π/3): the section at height z = 30t is the Minkowski combination (1 − t)S + tD of the square and the disc, of area (1 − t)²·1600 + t(1 − t)·160·15 + t²·225π (the mixed term the square's perimeter times the circle's radius)"
tex = '28000 + 2250\pi = 30\left(\tfrac{1600}{3} + \tfrac{160\cdot 15}{6} + \tfrac{225\pi}{3}\right)'

[area]
value = 6241.372840065616
exact = "no closed form (each cone's area is an elliptic integral): OCCT (6241.372839877755) and cad-kernel (6241.372840253477), built independently from this description, agree to 6e-11; the mean. The floor, the disc and the triangles are 1600 + 225π + 400√37"
+++

A transition piece, as a sheet-metal square-to-round is made, from a square at its foot to a
circle at its top: a loft from four straight sides to one round one.

The square is −20 ≤ x ≤ 20, −20 ≤ y ≤ 20 in the plane z = 0. The circle has radius 15 and lies in
the plane z = 30, centred on the z axis.

Each side of the square is joined by a flat triangle to the point of the circle nearest it: the
side from (−20, −20, 0) to (20, −20, 0) to the point (0, −15, 30), and so on round, the four
points being (0, −15, 30), (15, 0, 30), (0, 15, 30) and (−15, 0, 30). Each corner of the square is
joined to the quarter of the circle between the two points beside it by a piece of an oblique
cone, its apex the corner: every point of the quarter circle from (0, −15, 30) to (15, 0, 30) is
joined to the corner (20, −20, 0) by a straight line, and so on round. Each triangle meets the
cones on either side of it along a line from a corner of the square to one of the four points,
and the cone and the triangle are tangent there.

So the walls are four flat triangles and four conical pieces, alternating round, and the solid is
the convex hull of the square and the circle: its section at every height is the Minkowski
combination of the two, a square with rounded corners, the rounding growing from nothing at the
foot to the whole circle at the top. Its floor and top are flat.
