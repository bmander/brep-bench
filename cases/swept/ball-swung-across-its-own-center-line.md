+++
title = "A ball swung about an axis that passes through it"
tags = ["swept", "solid-sweep", "turn", "sphere", "canal", "spindle", "self-intersecting"]
origin = "geomsolver #721, box X9 (a ball reaching across the axis, a spindle torus)"

[volume]
value = 59.3331943709
exact = "no closed form: OCCT and cadrs agree to 8e-10; the mean of their values"

[area]
value = 75.5328704821
exact = "no closed form: OCCT and cadrs agree to 9e-10; the mean of their values"
+++

The solid a ball fills as it is swung partway about an upright line that passes through it.

The ball has radius 2 and its center is (1, 0, 0). It turns rigidly about the z axis, right-handed
(from +x toward +y), by 120°, its center running along an arc of radius 1. Make the solid of
everything the ball passes through.

The ball reaches across the turn's axis, so the tube about its center's arc is a piece of a spindle
torus: it passes through itself on the axis, and its inner part is buried inside the balls at the
two ends.
