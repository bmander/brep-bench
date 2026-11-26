+++
title = "A block with a square pocket grown by 1"
tags = ["offset", "grow", "planar", "concave-edge", "pocket", "corner", "adversarial"]
origin = "adversarial: offset/box-grown with a square pocket, whose rims meet its concave vertical edges"

[volume]
value = 3715.1681268319685
exact = "V₀ − [(w − 2d)²(D − d) + 64d − 8πd² + 8d³/3] = (10624 + 166π)/3, V₀ = abc + 2(ab + bc + ca)d + π(a + b + c)d² + 4πd³/3, a = b = 20, c = 6, w = 8, D = 3, d = 1 (64d − 8πd² + 8d³/3 = ∫₀ᵈ (w − 2√(d² − u²))² du)"
tex = 'V_0 - \Bigl[(w - 2d)^2(D - d) + \int_0^d \bigl(w - 2\sqrt{d^2 - u^2}\bigr)^2\,du\Bigr] = \tfrac{10624 + 166\pi}{3},\quad V_0 = abc + 2(ab + bc + ca)\,d + \pi(a + b + c)\,d^2 + \tfrac{4}{3}\pi d^3,\ a = b = 20,\ c = 6,\ w = 8,\ D = 3,\ d = 1'

[area]
value = 1643.8583772020568
exact = "the derivative in d of the volume: 1292 + 112π"
tex = '\tfrac{\partial V}{\partial d} = 1292 + 112\pi'
+++

The block from (0, 0, 0) to (20, 20, 6) with a square pocket cut down into its top: the square
from (6, 6) to (14, 14), its sides parallel to the block's and its corners square, from the top
down to a flat floor at z = 3. Grow it by 1: the solid of every point within 1 of the pocketed
block.

The block grows as a box does: each outer face moves 1 outward, each outer edge becomes a quarter
of a cylinder of radius 1, each outer corner an eighth of a ball. In the pocket, every point
within 1 of its walls or its floor is filled: the floor rises to z = 4 and the walls come in 1,
so the pocket left below the grown top is the square from (7, 7) to (13, 13), from z = 4 up to
z = 6, with sharp concave edges along its floor and its four vertical corners. The pocket's rim,
where its walls meet the block's top, is a convex edge and becomes a quarter of a cylinder of
radius 1 about it, so above z = 6 the pocket flares: at a height u above the top (0 ≤ u ≤ 1) the
opening left is the square from (6 + √(1 − u²), 6 + √(1 − u²)) to (14 − √(1 − u²),
14 − √(1 − u²)), its corners sharp, closing to the full 8 × 8 square at z = 7. At each of the
pocket's four corners the rims' two rounds meet in a sharp concave crease above the corner's
vertical edge.

The grown solid is the grown box, of volume V₀, less that opening.

**As a set:**

$$\Bigl([0,20]^2\times[0,6] \;\setminus\; [6,14]^2\times[3,7]\Bigr) \oplus B_{1}$$
