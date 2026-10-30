+++
title = "A turned shaft with a rounded shoulder"
tags = ["part", "revolve", "analytic", "fillet"]
origin = "geomsolver rust/examples/turned_shaft.sv"

[volume]
value = 120915.32332987145
exact = "π·20²·60 + π·12²·100 + 2π(4²·14 − π·4²/4·(16 − 16/(3π))) (Pappus) = 116672π/3 − 128π²"

[area]
value = 17470.591304910977
exact = "π·20² + 2π·20·60 + π(16² − 12²) + 2π·2π·(16 − 8/π) + 2π·12·96 + π·12² = 5360π + 64π²"
+++

A stepped shaft on the z axis, the solid swept by turning a section in the xz plane a full turn
about the z axis. In the xz plane (the first number is x and the second z), the section's outline
runs:

- from (0, 0) out to (20, 0), the end of the thick part;
- from (20, 0) up to (20, 60), the thick part's side;
- from (20, 60) in to (16, 60), the shoulder;
- along a quarter circle of radius 4 centred at (16, 64), from (16, 60) to (12, 64), bulging
  toward the corner (12, 60) (its middle is at (16 − 2√2, 64 − 2√2));
- from (12, 64) up to (12, 160), the thin part's side;
- from (12, 160) in to (0, 160), the end of the thin part;
- from (0, 160) down the axis back to (0, 0).

So it is a cylinder of radius 20 from z = 0 to z = 60 and a cylinder of radius 12 from z = 60 to
z = 160, with the inside corner where the shoulder meets the thin part rounded to radius 4: the
round is tangent to the shoulder face at radius 16 and to the thin part's side at z = 64.
