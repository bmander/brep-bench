+++
title = "A square-to-round transition three times larger"
tags = ["loft", "ruled", "transition", "unequal-sections", "hull", "cone", "adversarial"]
origin = "adversarial: loft/square-to-round three times larger"

[volume]
value = 946851.7537055799
exact = "756000 + 60750π = 90(14400/3 + 480·45/6 + 2025π/3): the section at height z = 90t is the Minkowski combination (1 − t)S + tD of the square and the disk, of area (1 − t)²·14400 + t(1 − t)·480·45 + t²·2025π (the mixed term the square's perimeter times the circle's radius)"
tex = '756000 + 60750\pi = 90\left(\tfrac{14400}{3} + \tfrac{480\cdot 45}{6} + \tfrac{2025\pi}{3}\right)'

[area]
value = 56172.355560590544
exact = "no closed form (each cone's area is an elliptic integral): 3² times loft/square-to-round's answer (the mean of OCCT's and cad-kernel's values, which agree to 6e-11), the solid being that one scaled by 3. The floor, the disk and the triangles are 14400 + 2025π + 3600√37"
+++

The transition piece of `loft/square-to-round`, three times larger: from a square at its foot to a
circle at its top, a loft from four straight sides to one round one.

The square is −60 ≤ x ≤ 60, −60 ≤ y ≤ 60 in the plane z = 0. The circle has radius 45 and lies in
the plane z = 90, centered on the z axis.

Each side of the square is joined by a flat triangle to the point of the circle nearest it: the
side from (−60, −60, 0) to (60, −60, 0) to the point (0, −45, 90), and so on around, the four
points being (0, −45, 90), (45, 0, 90), (0, 45, 90) and (−45, 0, 90). Each corner of the square is
joined to the quarter of the circle between the two points beside it by a piece of an oblique
cone, its apex the corner: every point of the quarter circle from (0, −45, 90) to (45, 0, 90) is
joined to the corner (60, −60, 0) by a straight line, and so on around. Each triangle meets the
cones on either side of it along a line from a corner of the square to one of the four points,
and the cone and the triangle are tangent there.

So the walls are four flat triangles and four conical pieces, alternating around, and the solid is
the convex hull of the square and the circle: its section at every height is the Minkowski
combination of the two, a square with rounded corners, the rounding growing from nothing at the
foot to the whole circle at the top. Its floor and top are flat.

**As a set:**

$$\operatorname{conv}\Bigl(\bigl([-60,60]^2\times\{0\}\bigr) \cup \{\,\rho \le 45,\ z =
90\,\}\Bigr)$$
