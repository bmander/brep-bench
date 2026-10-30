+++
title = "A V-belt pulley"
tags = ["part", "revolve", "boolean", "cut", "analytic"]
origin = "geomsolver rust/examples/solid_pulley.sv"

[volume]
value = 41490.56537700988
exact = "π(28² − 6²)·18 + π(12² − 6²)·10 − ∫₂₂²⁸ 2πr·(10/7)(r − 22) dr = 14544π − 9360π/7 = 92448π/7"

[area]
value = 10484.569630424683
exact = "2π(12² − 6²) + 2·2π·12·5 + 2π(28² − 12²) + 2π·6·28 + 2π·28·(18 − 60/7) + 2·π(22 + 28)·√(6² + (30/7)²) = 2600π + 600√74·π/7"
+++

A pulley turned about the z axis, symmetric about the plane z = 0:

- a hub, a cylinder of radius 12 from z = −14 to z = 14;
- a rim, a disc of radius 28 from z = −9 to z = 9, so the hub stands 5 proud of the rim on each
  side;
- a bore of radius 6 on the z axis, right through, from z = −14 to z = 14.

A V-shaped belt groove runs all the way round the rim. In the xz plane (the first number is x and
the second z) the groove is the part of the triangle with corners (22, 0), (29, −5) and (29, 5)
that lies inside the pulley, turned a full turn about the z axis and taken away. So the groove's
root is the circle of radius 22 at z = 0, 6 deep below the rim's face, and its two flanks are cones
that open 5 in z for every 7 outward in radius, meeting the rim's face (radius 28) at
z = ±30/7. The triangle reaches 1 past the rim's face so the groove opens cleanly; if you cut it
with a revolved triangle, keep it reaching past the rim like that.
