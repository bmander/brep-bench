+++
title = "A groove carried along a spline by a ball"
tags = ["swept", "solid-sweep", "canal", "spline", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/path.sv"

[volume]
value = 10173.828705378295
exact = "no closed form (the Bézier's arc length is an integral): OCCT and cad-kernel, built independently from this description, agree to 9e-12 (10173.828705331523 and 10173.828705425067; the mean)"

[area]
value = 4026.6392331153256
exact = "no closed form (the Bézier's arc length is an integral): OCCT and cad-kernel, built independently from this description, agree to 3e-11 (4026.6392330649683 and 4026.6392331656834; the mean)"
+++

A block, the box from (−6, −6, −8) to (36, 26, 0), with a groove cut into its top face by a ball of
radius 3 carried along a curve. The ball's centre follows the cubic Bézier curve in the top face
(z = 0) whose control points are (0, 0, 0), (10, 0, 0), (20, 20, 0) and (30, 20, 0), from its first
end to its last. Cut from the block everything the ball passes through.

The groove is half of the tube of radius 3 about the curve, closed at each end by a quarter of the
ball. The curve's tightest turn has a radius of about 7.5, more than the ball's, so the tube does not
fold on itself.
