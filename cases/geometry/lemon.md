+++
title = "A lemon: a circular segment turned about its chord"
tags = ["geometry", "torus", "degenerate", "apex", "pole", "revolution"]

[volume]
value = 57.299667477372516
exact = "2π times the segment's first moment about its chord, (2/3)(r² − d²)^{3/2} − d·A, A = r²θ − d√(r² − d²), θ = arccos(d/r): 2π(236/3 − 75θ), r = 5, d = 3, θ = arccos(3/5)"
tex = '2\pi\left(\tfrac{236}{3} - 75\theta\right),\quad \theta = \arccos\tfrac35'

[area]
value = 76.53638161221481
exact = "the arc turned, 2πr∫(r cos φ − d) dφ over |φ| ≤ θ = 4πr(r sin θ − dθ): 20π(4 − 3θ)"
tex = '20\pi\left(4 - 3\theta\right),\quad \theta = \arccos\tfrac35'
+++

The inner part of a spindle torus: a lemon, pointed at both ends. In the xz plane take the circle
of radius 5 centred at (−3, 0, 0), and the part of its disc with x ≥ 0: a circular segment, cut
off by the z axis as its chord, from (0, 0, −4) to (0, 0, 4), and reaching out to (2, 0, 0). Turn
it a whole turn about the z axis.

The solid is the region the tube of `geometry/spindle-torus` (middle circle radius 3, tube radius
5) sweeps twice: every point within 5 of every point of the circle of radius 3 about the z axis in
the plane z = 0. Its surface is the segment's arc turned, which meets the axis at (0, 0, ±4) at an
angle, so the lemon comes to a sharp point at each end, a cone-like apex where the surface has no
tangent plane, and is widest, radius 2, at z = 0.
