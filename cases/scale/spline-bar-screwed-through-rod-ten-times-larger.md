+++
title = "A rod threaded by a bar whose section is a spline, ten times larger"
tags = ["swept", "solid-sweep", "screw", "helix", "bspline", "prism", "free-form", "boolean", "cut", "large", "scale", "adversarial"]
origin = "adversarial: swept/spline-bar-screwed-through-rod ten times larger"

[volume]
value = 1335319.592427
exact = "no closed form: 10³ times swept/spline-bar-screwed-through-rod's answer (the mean of cad-kernel's, next.BREP's and cadrs's values, which agree to 1e-9), the solid being that one scaled by 10 about the origin"

[area]
value = 81056.8231802
exact = "no closed form: 10² times swept/spline-bar-screwed-through-rod's answer (the mean of cad-kernel's, next.BREP's and cadrs's values, which agree to 3e-10), the solid being that one scaled by 10 about the origin"
+++

The rod and bar of `swept/spline-bar-screwed-through-rod`, ten times larger: a rod, the cylinder of
radius 50 about the z axis from z = 0 to z = 200, threaded by a bar whose section is a closed
B-spline curve, carried along a screw.

The bar's section lies in the plane y = 0: it is the closed, uniform, periodic cubic B-spline curve
(non-rational, knots evenly spaced, each span the cubic blend of four consecutive control points)
whose seven control points, in (x, z) and in order round the loop, are (35, 0), (45, −15),
(80, −15), (120, −10), (120, 10), (80, 15) and (45, 15). Its point nearest the rod's axis is
(115/3, 0, 0), about 11.7 inside the rod. The bar is everything within that curve carried from
y = −80 to y = 80: a prism, its sides one surface of extrusion of the spline.

The screw is about the z axis: the tool turns right-handed about it (from +x toward +y) and rises 60
for every whole turn, rigidly. It runs from a turn of −90° (lowered 15, the tool wholly below the
rod) to a turn of 1350° (raised 225, wholly above it). Cut from the rod everything the bar passes
through.

Its contact under the screw has no closed form on a spline: it must be found on the surface
numerically, at each ruling.
