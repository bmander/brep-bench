+++
title = "A ball pierced off centre"
tags = ["boolean", "cut", "analytic", "off-axis"]
origin = "geomsolver rust/examples/pierced_sphere.sv"

[volume]
value = 29240.844398085323
exact = "Solvent and OCCT agree (independent builds from this description)"

[area]
value = 6171.861947555933
exact = "Solvent and OCCT agree (independent builds from this description)"
+++

A ball of radius 20 centred on the origin, with a round hole of radius 6 bored right through it
parallel to the y axis. The hole's axis is the line x = 5, z = 0, so the hole runs from x = −1 to
x = 11 and stays inside the ball's outline: it enters and leaves through the sphere and never
breaks out of its side. If you cut the hole with a cylinder, make it run past the ball at both
ends.
