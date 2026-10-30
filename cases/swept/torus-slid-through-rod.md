+++
title = "A rod grooved by a wheel slid along it"
tags = ["swept", "solid-sweep", "slide", "torus", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/slide.sv"

[volume]
value = 1493.7545729515407
exact = "πR²h − h·lens(R = 5, r = 2, d = 5.5), h = 20"
tex = '\pi R^2 h - h\,A_{\text{lens}}(R = 5,\ r = 2,\ d = 5.5),\quad h = 20'

[area]
value = 794.3608989284018
exact = "2(πR² − lens) + h(2πR − 2αR) + h·2βr, cos α = (d² + R² − r²)/2dR, cos β = (d² + r² − R²)/2dr"
tex = '2\left(\pi R^2 - A_{\text{lens}}\right) + h(2\pi R - 2\alpha R) + 2\beta r h,\quad \cos\alpha = \frac{d^2 + R^2 - r^2}{2dR},\ \cos\beta = \frac{d^2 + r^2 - R^2}{2dr}'
+++

A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, grooved along its length by a
wheel slid past it. The wheel is a torus: its axis is parallel to the y axis, the circle through its
tube's centres has radius 6, and its tube has radius 2. It starts with its centre at (11.5, 0, −9),
so the part of its tube nearest the rod is centred 5.5 from the rod's axis and reaches 1.5 into it,
and the whole wheel is below the rod. It slides without turning along the z axis, up to centre
(11.5, 0, 29), where the whole wheel is above the rod. Cut from the rod everything the wheel passes
through.
