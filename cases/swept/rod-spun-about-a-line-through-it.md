+++
title = "A rod spun partway about a line through it"
tags = ["swept", "solid-sweep", "turn", "cylinder", "pole", "axis-crossing"]
origin = "geomsolver #721, box D3 (crossing a pole under a turn about a meeting axis)"

[volume]
value = 41.68570617465
exact = "no closed form: next.BREP and cadrs agree to 6e-9; the mean of their values"

[area]
value = 111.350261639
exact = "no closed form: next.BREP and cadrs agree to 7e-9; the mean of their values"
+++

The solid a rod fills as it is spun partway about an upright line that passes through it.

The rod is a solid cylinder of radius 0.5 whose axis is the x axis, from x = −3 to x = 5, its ends
flat. The z axis crosses it square to its axis.

It turns rigidly about the z axis, right-handed (from +x toward +y), by 120°. Make the solid of
everything the rod passes through.

The turn's axis crosses the rod's. Every circle of the rod about its own axis touches at its top and
bottom points, z = ±0.5, except the one at x = 0, which the turn moves along the rod's own axis and
which touches all round: there the two roots cross. The rod's two arms sweep two fans, of radii 5
and 3, that meet about the axis.
