+++
title = "A groove swung by a ball"
tags = ["swept", "solid-sweep", "canal", "torus", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/ball.sv"

[volume]
value = 4284.972013405971
exact = "24·24·8 − ½(πr²·Rθ + 4πr³/3), r = 3, R = 12, θ = π/2"

[area]
value = 2012.829877572684
exact = "box − (2rRθ + πr²) + (πrRθ + 2πr²)"
+++

A block, the box from (−6, −6, −8) to (18, 18, 0), with a curved groove cut into its top face by a
ball. The ball has radius 3 and its centre starts at (12, 0, 0), on the top face, so it is half sunk
in the block. It swings a quarter turn about the z axis, right-handed (from +x towards +y), so that
its centre ends at (0, 12, 0); its centre stays in the top face all the way. Cut from the block
everything the ball passes through.

The groove is a round-bottomed channel: a quarter of a torus's tube below the top face, closed at
each end by a quarter of the ball.
