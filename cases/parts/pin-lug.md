+++
title = "A pin lug"
tags = ["part", "boolean", "cut", "analytic"]
origin = "geomsolver rust/examples/pin_lug.sv"

[volume]
value = 29989.380701702525
exact = "80·40·10 − π·8²·10"

[area]
value = 8900.530964914875
exact = "2(80·40 − π·8²) + 2(80 + 40)·10 + 2π·8·10 = 8800 + 32π"
+++

A flat lug: a plate from (0, −5, 0) to (80, 5, 40), so it is 80 long in x, 40 tall in z and 10
thick in y. A round pin hole of radius 8 goes straight through its thickness, along y, its axis
the line x = 60, z = 20. If you cut the hole with a cylinder, make it run past both faces of the
plate.
