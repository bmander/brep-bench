+++
title = "A hollow square duct elbow"
tags = ["sweep", "planar-path", "hollow", "analytic"]
origin = "geomsolver rust/examples/solid_elbow.sv"

[volume]
value = 6031.857894892429
exact = "(18² − 14²)·30·π/2 = 1920π (the section, 128, carried round its centre's radius 30)"
tex = '\left(18^2 - 14^2\right)\cdot 30\cdot\tfrac{\pi}{2} = 1920\pi'

[area]
value = 6287.857894892418
exact = "2·128 + 2·(π/4)(39² − 21²) + (π/2)·18·(39 + 21) + 2·(π/4)(37² − 23²) + (π/2)·14·(37 + 23) = 256 + 1920π"
tex = '2\cdot 128 + 2\cdot\tfrac{\pi}{4}\left(39^2 - 21^2\right) + \tfrac{\pi}{2}\cdot 18(39 + 21) + 2\cdot\tfrac{\pi}{4}\left(37^2 - 23^2\right) + \tfrac{\pi}{2}\cdot 14(37 + 23) = 256 + 1920\pi'
+++

A hollow square duct bent through a quarter turn. Its centreline is an arc of radius 30 in the xy
plane about the z axis, from (30, 0, 0) to (0, 30, 0).

At every point of the arc the section lies in the vertical plane through the z axis and that
point. It is a square 18 × 18 centred on the arc, two of its sides horizontal (at z = −9 and
z = 9) and two vertical (at 21 and 39 from the z axis), with a square hole 14 × 14 at its centre,
so the wall is 2 thick all round. The section is the same all the way round: the duct is that
hollow square, standing in the xz plane at x = 30, turned a quarter turn about the z axis, from +x
towards +y. Its two ends are flat, in the xz and yz planes.
