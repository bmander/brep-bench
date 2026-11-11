+++
title = "A groove cut by a bar drafted both ways from a parting line"
tags = ["swept", "solid-sweep", "turn", "prism", "draft", "crease", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/creased.sv"

[volume]
value = 38.02835463795
exact = "no closed form: OCCT and cadrs agree to 2e-10; the mean of their values"

[area]
value = 84.178253743
exact = "no closed form: OCCT and cadrs agree to 2e-10; the mean of their values"
+++

A curved block grooved by a bar drafted both ways from the plane its section is drawn in, as a tool
forged or cast from a parting line is, turned past the block.

The block is a sector of a ring about the z axis: everything with distance from the z axis between 8
and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The bar is drawn in the plane through the origin spanned by u = (1, 0, 0) and v = (0, cos 20°, −sin
20°), leaning 20° across the turn; its normal is n = (0, sin 20°, cos 20°). The bar is every point a
u + b v + h n with |h| ≤ 4, 10 + |h| tan 2° ≤ a ≤ 13 − |h| tan 2° and |b| ≤ 2 − |h| tan 2°: the
rectangle 10 ≤ a ≤ 13, |b| ≤ 2 in that plane, its four sides drafted 2° inward both ways from it.
Each side folds at the plane in a convex crease.

It turns rigidly about the z axis, right-handed (from +x toward +y), from −140° to +140°: it starts
clear of the block, passes through it, and ends clear of it. Cut from the block everything the bar
passes through.
