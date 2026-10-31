+++
title = "A slab pinched thin on four Bézier guides"
tags = ["loft", "guides", "spline", "bezier", "thin", "pythagorean-hodograph"]

[volume]
value = 2436.0685714285714
exact = "∫ 4pq dz = ∫₀¹ 4p(t)q(t)z′(t) dt = 426312/175, where the guide is (p, q, z) = (12 + g, 16/5 − g, 24t + 9t² − 9t³), g = 18t − 27t²: a polynomial in the guide's parameter t"
tex = '\int 4pq\,dz = \int_0^1 4\,p(t)\,q(t)\,z^\prime(t)\,dt = \tfrac{426312}{175},\quad (p, q, z) = \left(12 + g,\ \tfrac{16}{5} - g,\ 24t + 9t^2 - 9t^3\right),\ g = 18t - 27t^2'
[area]
value = 2124.0
exact = "24·6.4 + 6·24.4 + ∫₀¹ 4(p + q)·3(9 + (1 − 3t)²) dt = 300 + 4·15.2·30: the wall facing +x, the surface x = p between y = ±q, has area ∫ 2q·|(p′, z′)| dt, and the guide seen along y, (p, z), and along x, (q, z), are both Pythagorean-hodograph cubics, |(p′, z′)| = |(q′, z′)| = 3(9 + (1 − 3t)²); so the four walls are ∫ 4(p + q)·3(9 + (1 − 3t)²) dt, with p + q = 15.2 throughout"
tex = '24\cdot 6.4 + 6\cdot 24.4 + \int_0^1 4(p + q)\cdot 3\left(9 + (1 - 3t)^2\right)dt = 300 + 4\cdot 15.2\cdot 30'
+++

A solid lofted through rectangles, its shape set by four guide curves: a slab that is pinched
almost to nothing on its way up, while its long sides turn from x to y.

Every section of the solid at a height z from 0 to 24 is a rectangle in the plane at that height,
centred on the z axis, with its sides parallel to the x and y axes, and its four corners lie on
the four guides. The guide through the corner with x and y positive is the cubic Bézier curve with
control points (12, 3.2, 0), (18, −2.8, 8), (15, 0.2, 19) and (3, 12.2, 24); the other three are
its mirror images in the xz plane, the yz plane and both (its control points with y, x, or both
negated). Along each guide z rises steadily, so each height meets each guide once.

So the rectangle begins as 24 × 6.4 (x from −12 to 12, y from −3.2 to 3.2) at z = 0. As it rises
it grows longer in x and thinner in y, until at z = 26/3 (the guides' parameter t = 1/3) it is
30 × 0.4, its thinnest; then its length along x shrinks while its width along y grows, and it ends
as 6 × 24.4 at z = 24. The width along y is never less than 0.4.

Each of the four walls is swept by one side of the rectangle, a straight horizontal segment
between two neighbouring guides: the wall facing +x is made of the segments parallel to the y
axis from each point (p, −q, z) of one guide to the point (p, q, z) of the other, and the wall
facing +y of the segments parallel to the x axis from (−p, q, z) to (p, q, z). The ends are flat.
