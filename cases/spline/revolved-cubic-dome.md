+++
title = "A dome turned from a cubic Bézier curve"
tags = ["spline", "revolve", "bezier", "pole", "overhang"]

[volume]
value = 2464.7140764263468
exact = "π∫ x² dz along the curve = π∫₀¹ x(t)² z′(t) dt = 27459π/35 (a polynomial in t)"
tex = '\pi\int x^2\,dz = \pi\int_0^1 x(t)^2 z^\prime(t)\,dt = \tfrac{27459\pi}{35}'

[area]
value = 983.9468191043231
exact = "π·9² + 2π∫₀¹ x(t)|c′(t)| dt = 81π + 2π·1161/10 = 1566π/5: the curve is a Pythagorean-hodograph cubic, |c′(t)| = 3(5 − 4t + 8t²), so the integrand is a polynomial"
tex = '\pi\cdot 9^2 + 2\pi\int_0^1 x(t)\,|c^\prime(t)|\,dt = 81\pi + 2\pi\cdot\tfrac{1161}{10} = \tfrac{1566\pi}{5},\quad |c^\prime(t)| = 3\left(5 - 4t + 8t^2\right)'
+++

A dome made by turning a plane region a full turn about the z axis. The region lies in the xz
plane, on the side x ≥ 0, and is bounded by three pieces: the segment of the z axis from (0, 0, 0)
to (0, 0, 10); the segment of the x axis from (0, 0, 0) to (9, 0, 0); and the cubic Bézier curve
whose four control points are (9, 0, 0), (12, 0, 4), (9, 0, 10) and (0, 0, 10).

The curve leaves the base's rim heading up and outward, along (3, 0, 4), so the dome overhangs
its flat round base (radius 9), bulging to 10.125 from the axis; it then curls in over the
top and meets the z axis at (0, 0, 10) heading straight at it, along −x. So the domed face closes
smoothly over a single point on the axis, with no spike, and the dome's only other face is the
flat disc it stands on.
