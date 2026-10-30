+++
title = "A hollow square reducer"
tags = ["sweep", "loft", "hollow", "ruled"]
origin = "geomsolver rust/examples/solid_loft.sv"

[volume]
value = 5119.999999999596
exact = "(40/3)(24² + 12² + 24·12) − (40/3)(20² + 8² + 20·8) = 13440 − 8320 = 5120"

[area]
value = 5433.279594536134
exact = "(24² − 20²) + (12² − 8²) + 4·((24 + 12)/2 + (20 + 8)/2)·√(40² + 6²) = 256 + 256√409"
+++

A hollow square tube that tapers along the y axis, from y = 0 to y = 40.

Its sections are squares centred on the y axis, with sides parallel to the x and z axes, each with
a square hole at its centre leaving a wall 2 thick. At y = 0 the outside is 24 × 24 (x and z from
−12 to 12) and the hole 20 × 20; at y = 40 the outside is 12 × 12 and the hole 8 × 8. In
between, the sizes run linearly with y, so each corner, outside and in, runs in a straight line
from one end to the other and the eight long faces are flat trapezoids. The two ends are flat.
