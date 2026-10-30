+++
title = "A hollow square duct narrowing round a bend"
tags = ["sweep", "planar-path", "hollow", "variable-section"]
origin = "geomsolver rust/examples/solid_bend.sv"

[volume]
value = 4900.88453960008
exact = "∫ 30·A dθ over the quarter turn, A = (2h)² − (2h − 4)² = 16(h − 1), h running 9 → 6: 30·16·6.5·π/2 = 1560π"

[area]
value = 5119.172836922541
exact = "208 + (π/6)·[18F(39) − 12F(36) + 12F(24) − 18F(21) + 14F(37) − 8F(34) + 8F(26) − 14F(23)], F(ρ) = ½(ρ√(ρ² + c²) + c²·asinh(ρ/c)), c = 6/π (the half-width's change per radian); 208 = 128 + 80 the two ends"
+++

A hollow square duct that narrows as it bends through a quarter turn. Its centreline is an arc of
radius 30 in the xy plane about the z axis, from (30, 0, 0) to (0, 30, 0).

At every point of the arc the section lies in the vertical plane through the z axis and that
point. It is a square centred on the arc, two of its sides horizontal and two vertical (one side
facing the z axis), with a square hole at its centre, so that the wall is 2 thick all round. The
section's outer half-width h runs linearly with the angle turned, from 9 at the start to 6 at the
end: at an angle θ from the xz plane, h = 9 − 3·θ/90°. So the outside of the section reaches
from 30 − h to 30 + h out from the z axis and from z = −h to z = h, and the hole is the square of
half-width h − 2: 18 × 18 with a hole 14 × 14 at the start, in the xz plane at x = 30, and
12 × 12 with a hole 8 × 8 at the end, in the yz plane at y = 30.

The sides of every section are straight, so each of the eight long faces is swept by one side of
the section: a straight segment that stays horizontal (or vertical) in its plane through the z
axis while its ends move along the spirals the section's corners trace. The two ends are flat.
