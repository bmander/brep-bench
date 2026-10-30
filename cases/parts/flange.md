+++
title = "A bored mounting flange with six bolt holes"
tags = ["part", "revolve", "boolean", "cut", "analytic", "pattern"]
origin = "geomsolver rust/examples/solid_flange.sv"

[volume]
value = 22135.66183719368
exact = "π(32² − 8²)·6 + π(15² − 8²)·10 − 6·π·3²·6 = 7046π"

[area]
value = 9324.246995854506
exact = "π(32² − 8²) + π(32² − 15²) + π(15² − 8²) + 2π·32·6 + 2π·15·10 + 2π·8·16 − 12π·3² + 6·2π·3·6 = 2968π"
+++

A round mounting flange on the z axis, its base on the plane z = 0:

- a plate, a disc of radius 32 from z = 0 to z = 6;
- a hub, a cylinder of radius 15 standing on the plate from z = 6 to z = 16;
- a bore of radius 8 on the z axis, right through the plate and the hub from z = 0 to z = 16.

Six bolt holes of radius 3 go straight through the plate along z. Their axes stand on a circle of
radius 24 about the z axis, one every 60°: at (x, y) = (24, 0), (12, 12√3), (−12, 12√3), (−24, 0),
(−12, −12√3) and (12, −12√3). The holes stay clear of the hub (their nearest point is 21 from the
axis, the hub's side 15) and of the plate's rim (furthest point 27, rim 32). If you cut the holes
with cylinders, make them run past both faces of the plate.
