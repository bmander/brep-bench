+++
title = "A thin blade twisted through a full turn"
tags = ["loft", "guides", "twisted", "thin", "helix", "ruled"]

[volume]
value = 960.0
exact = "40·0.4·60: every section is the same rectangle, turned"
tex = '40\cdot 0.4\cdot 60'

[area]
value = 7351.884474938799
exact = "2·40·0.4 + 2W(20) + 2W(0.2), W(a) = 60∫₋ₐᵃ √(1 + k²u²) du = 60(a√(1 + k²a²) + asinh(ka)/k), k = 2π/60 the turn per unit of height: a point of a wall u along its side from the side's middle has area element √(1 + k²u²) du dz, whatever the side's distance from the axis; W(20) is each broad face, W(0.2) each narrow edge"
tex = '2\cdot 40\cdot 0.4 + 2W(20) + 2W(0.2),\quad W(a) = 60\int_{-a}^{a}\sqrt{1 + k^2u^2}\,du = 60\left(a\sqrt{1 + k^2a^2} + \frac{\operatorname{asinh}(ka)}{k}\right),\ k = \tfrac{2\pi}{60}'
+++

A thin flat blade twisted through a whole turn: a solid lofted through one thin rectangle, turned
steadily as it rises, its four corners running along four helical guides.

The solid runs from z = 0 to z = 60. Its section at every height z is the rectangle 40 × 0.4,
centred on the z axis, turned about the z axis right-handed (from +x towards +y) by 6° for every
unit of height. So it begins as −20 ≤ x ≤ 20, −0.2 ≤ y ≤ 0.2 at z = 0; at z = 15 its long sides
are parallel to the y axis; at z = 30 it lies along x again; and at z = 60 it is back where it
began, having turned 360°.

Each corner of the rectangle runs along a helix of radius √(20² + 0.2²) about the z axis, rising 60
in a turn: the four guides. Each of the four walls is swept by one side of the rectangle, a
straight horizontal segment between two neighbouring helices. The two broad faces, 0.4 apart,
are swept by the long sides, each 0.2 from the axis; at their outer edges they are steep, the
helices there rising only 60 in a circumference of about 126. The two narrow faces, 0.4 wide, are
swept by the short sides, 20 from the axis. The ends are flat.
