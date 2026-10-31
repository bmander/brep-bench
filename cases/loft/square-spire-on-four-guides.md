+++
title = "A square spire on four Bézier ridges"
tags = ["loft", "guides", "spline", "bezier", "apex", "pythagorean-hodograph"]

[volume]
value = 9423.771428571428
exact = "∫ 4g² dz = ∫₀¹ 4g(t)²z′(t) dt = 329832/35, where the ridge is (g, g, z) = (9 + 18t − 27t², 9 + 18t − 27t², 24t + 9t² − 9t³): a polynomial in its parameter t"
tex = '\int 4g^2\,dz = \int_0^1 4\,g(t)^2\,z^\prime(t)\,dt = \tfrac{329832}{35},\quad g = 9 + 18t - 27t^2,\ z = 24t + 9t^2 - 9t^3'

[area]
value = 2397.6
exact = "18² + 4∫₀¹ 2g·3(9 + (1 − 3t)²) dt = 324 + 10368/5 = 11988/5: the wall facing +x, the surface x = g between y = ±g, has area ∫ 2g·|(g′, z′)| dt, and the ridge seen along y, (g, z), is a Pythagorean-hodograph cubic, |(g′, z′)| = 3(9 + (1 − 3t)²)"
tex = '18^2 + 4\int_0^1 2g\cdot 3\left(9 + (1 - 3t)^2\right)dt = 324 + \tfrac{10368}{5} = \tfrac{11988}{5}'
+++

A square spire, swelling and then closing to a point: a solid lofted from a square at its foot to
a single point at its top, its shape set by four guide curves, its ridges.

Every section of the solid at a height z from 0 to 24 is a square in the plane at that height,
centred on the z axis, with its sides parallel to the x and y axes, and its four corners lie on
the four ridges. The ridge through the corner with x and y positive is the cubic Bézier curve with
control points (9, 9, 0), (15, 15, 8), (12, 12, 19) and (0, 0, 24), in the vertical plane x = y;
the other three are its mirror images in the xz plane, the yz plane and both (its control points
with y, x, or both negated). Along each ridge z rises steadily, so each height meets each ridge
once.

So the square begins as 18 × 18 at z = 0, swells to 24 × 24 at z = 26/3 (the ridges' parameter
t = 1/3), and then narrows to nothing: the four ridges meet at the apex (0, 0, 24), arriving there
at a slant, so the spire comes to a sharp point.

Each of the four walls is swept by one side of the square, a straight horizontal segment between
two neighbouring ridges: the wall facing +x is made of the segments parallel to the y axis from
each point (g, −g, z) of one ridge to the point (g, g, z) of the other. The four walls meet only
along the ridges and at the apex. The floor is flat.
