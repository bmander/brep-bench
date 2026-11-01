+++
title = "A spindle torus"
tags = ["geometry", "torus", "degenerate", "self-intersecting", "pinch"]

[volume]
value = 1537.7403276407763
exact = "2π times the first moment about the axis of the disc's part with x ≥ 0: 2π(πr²R − R·A + (2/3)(r² − R²)^{3/2}), A = r²θ − R√(r² − R²) the part cut off, θ = arccos(R/r): 2π(75(π − θ) + 236/3), R = 3, r = 5, θ = arccos(3/5)"
tex = '2\pi\left(75(\pi - \theta) + \tfrac{236}{3}\right),\quad \theta = \arccos\tfrac35'

[area]
value = 668.7126456775763
exact = "the outer arc turned, 2π∫ρ ds = 4πr(Rφ + r sin φ), φ = π − θ, sin φ = 4/5: 20π(3(π − θ) + 4)"
tex = '20\pi\left(3(\pi - \theta) + 4\right),\quad \theta = \arccos\tfrac35'
+++

A solid spindle torus centred on the origin, about the z axis: the circle through the middle of
its tube has radius 3, and the tube has radius 5, more than the middle circle's. The solid is
every point within 5 of that circle.

The tube is wider than its circle, so it crosses the axis: the torus's surface passes through
itself, and the solid is what the disc (x − 3)² + z² ≤ 25 of the xz plane fills as it turns a whole
turn about the z axis, counted once. That is the turn of the disc's part with x ≥ 0: an apple
shape, bounded only by the outer arc of the tube's circle, turned, which meets the axis at
(0, 0, ±4) in a dimple above and below. The inner part of the torus's surface, the lemon the tube
sweeps where it crosses the axis (`geometry/lemon`), lies inside the solid and is no part of its
boundary.
