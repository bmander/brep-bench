+++
title = "A disc with six holes indexed around it"
tags = ["boolean", "cut", "analytic", "pattern", "sequence"]
origin = "geomsolver rust/examples/solid_indexed_pattern.sv"

[volume]
value = 5906.194188748813
exact = "π·20²·5 − 6·π·2²·5 = 1880π"

[area]
value = 3367.7873246482577
exact = "2(π·20² − 6·π·2²) + 2π·20·5 + 6·2π·2·5 = 1072π"
+++

A round disc of radius 20 on the z axis, from z = −5 to z = 0. Six round holes of radius 2 go
straight through it along z, their axes on the circle of radius 12 about the z axis: the first at
(12, 0), and each of the others the first turned about the z axis by 60°, 120°, 180°, 240° and
300°. Cut them one after another, each hole its own cut. If you cut a hole with a cylinder, make
it run past both faces of the disc.
