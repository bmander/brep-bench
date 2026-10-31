+++
title = "A square tapering through a half turn on four spiral guides"
tags = ["loft", "guides", "twisted", "tapered", "ruled", "spiral"]

[volume]
value = 17333.333333333332
exact = "∫ s² dz = 40(30² + 30·10 + 10²)/3 = 52000/3: the section at height z is a square of side s = 30 − z/2, its turn changing nothing of its area"
tex = '\int s^2\,dz = \tfrac{40}{3}\left(30^2 + 30\cdot 10 + 10^2\right) = \tfrac{52000}{3},\quad s = 30 - \tfrac{z}{2}'

[area]
value = 4643.764402962483
exact = "30² + 10² + 4W, W = Σ± [G(kh₁ ± c) − G(kh₀ ± c)]/(k²c), G(x) = ((1 + x²)^{3/2}/3 + x·asinh x − √(1 + x²))/2: h = 15 − z/4 the half-side, h₀ = 15, h₁ = 5, c = dh/dz = −1/4, and k = π/40 the turn per unit of height. A point of a wall at height z, u along the side from its middle, has area element √(1 + (ku − c)²) du dz, so W = ∫₀⁴⁰∫₋ₕʰ √(1 + (ku − c)²) du dz"
tex = '30^2 + 10^2 + 4W,\quad W = \int_0^{40}\!\!\int_{-h}^{h}\sqrt{1 + (ku - c)^2}\,du\,dz = \sum_{\pm}\frac{G(kh_1 \pm c) - G(kh_0 \pm c)}{k^2 c},\quad G(x) = \tfrac{1}{2}\left(\tfrac{1}{3}(1 + x^2)^{3/2} + x\operatorname{asinh}x - \sqrt{1 + x^2}\right),\ k = \tfrac{\pi}{40},\ c = -\tfrac{1}{4}'
+++

A square column that tapers as it twists: a solid lofted from a large square at its foot to a
small one at its top, turning half a turn on the way, its four corners running along four spiral
guides.

The solid runs from z = 0 to z = 40. Its section at every height z is a square in the plane at
that height, centred on the z axis, of side 30 − z/2, and turned about the z axis, right-handed
(from +x towards +y), by 4.5° for every unit of height. So it begins as the square
−15 ≤ x ≤ 15, −15 ≤ y ≤ 15 at z = 0; at z = 10 it is 25 across and has turned 45°, its corners on
the axes; and it ends at z = 40 as the square −5 ≤ x ≤ 5, −5 ≤ y ≤ 5, having turned 180°.

Each corner of the square runs along a conical spiral, the four guides: the corner that starts at
(15, 15, 0) is, at height z, (15 − z/4)√2 from the z axis, at 45° + 4.5°·z from the +x direction.
Each of the four walls is swept by one side of the square, a straight horizontal segment between
two neighbouring guides, so the walls are twisted ruled surfaces. The ends are flat: the square of
side 30 at z = 0 and the square of side 10 at z = 40.
