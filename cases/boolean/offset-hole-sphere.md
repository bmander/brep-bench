+++
title = "A ball pierced off centre"
tags = ["boolean", "cut", "analytic", "off-axis"]
origin = "geomsolver rust/examples/pierced_sphere.sv"

[volume]
value = 29240.844399008805
exact = "no closed form (an off-axis hole needs elliptic integrals): OCCT and arris, built independently from this description, agree to 7e-11"

[area]
value = 6171.861947620873
exact = "no closed form: OCCT and arris, built independently from this description, agree to 5e-11"
+++

A ball of radius 20 centred on the origin, with a round hole of radius 6 bored right through it
parallel to the y axis. The hole's axis is the line x = 5, z = 0, so the hole runs from x = −1 to
x = 11 and stays inside the ball's outline: it enters and leaves through the sphere and never
breaks out of its side. If you cut the hole with a cylinder, make it run past the ball at both
ends.
