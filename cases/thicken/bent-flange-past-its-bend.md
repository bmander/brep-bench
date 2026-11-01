+++
title = "A bent sheet thickened inside its bend past the bend's radius"
tags = ["thicken", "sheet", "analytic", "cylinder", "sheet-metal", "self-intersection"]

[volume]
value = 497.1327412287184
exact = "w(2lt − (t − R)² + πR²/4), l = 10, R = 2, t = 3, w = 8"
tex = 'w\left(2lt - (t - R)^2 + \tfrac{\pi}{4}R^2\right),\quad l = 10,\ R = 2,\ t = 3,\ w = 8'

[area]
value = 501.41592653589794
exact = "2(2lt − (t − R)² + πR²/4) + w(4l + 2R + πR/2)"
tex = '2\left(2lt - (t - R)^2 + \tfrac{\pi}{4}R^2\right) + w\left(4l + 2R + \tfrac{\pi}{2}R\right)'
+++

The bent sheet of `thicken/bent-flange`: the profile in the xz plane that runs along the line from
(−10, 0, 0) to (0, 0, 0), then round the quarter circle of radius 2 about (0, 0, 2) to (2, 0, 2),
then up the line to (2, 0, 12), swept along +y from y = 0 to y = 8. It is three faces, meeting
tangentially: a flat floor, a bend that is a quarter of a cylinder of radius 2 about the line
through (0, ·, 2) along y, and a flat upright wall.

Thicken it by 3 to the inside of the bend, toward the bend's axis: above the floor and in front of
the wall, to −x. That is more than the bend's radius, 2. The thickened sheet is every point p + s·n,
for p on the sheet, n its unit normal on that side, and s from 0 to 3.

The bend's face, moved 3 toward its axis, would pass through the axis and turn inside out, a
cylinder of radius −1; all of it lies inside the solid and is no part of its boundary. The floor
moved up 3 and the wall moved in 3 meet instead at a concave edge along y at (−1, y, 3), and the
bend fills the quarter disc of radius 2 behind it. The section in any plane y = const is the region
bounded by the profile and the polyline from (2, 12) through (−1, 12), (−1, 3) and (−10, 3) to
(−10, 0), each given as (x, z).
