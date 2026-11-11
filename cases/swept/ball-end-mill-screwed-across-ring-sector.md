+++
title = "A helical groove milled by a ball-end mill screwed across a block"
tags = ["swept", "solid-sweep", "screw", "helix", "sphere", "cylinder", "canal", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/ball_end_mill.sv"

[volume]
value = 109.195626256
exact = "no closed form: OCCT and cadrs agree to 2e-9; the mean of their values"

[area]
value = 242.2865388095
exact = "no closed form: OCCT and cadrs agree to 2e-9; the mean of their values"
+++

A curved block with a helical groove milled into it by a ball-end mill screwed across it.

The block is a sector of a ring about the z axis: everything with distance from the z axis between 8
and 12, with z between −2 and 2, and with polar angle between −30° and 30° (measured from +x toward
+y).

The mill is a solid of revolution about the vertical line through (10, 0): the lower half of the
ball of radius 1 centered at (10, 0, 0), under a cylinder of radius 1 from z = 0 up to z = 3, flat
on top.

It is carried along a screw about the z axis: it turns right-handed about it (from +x toward +y) and
rises 3 for every whole turn, rigidly, from a turn of −60° (lowered 0.5) to one of +60° (raised
0.5). It starts clear of the block, passes wholly through it, and ends clear of it; its top stands
out above the block all the way. Cut from the block everything the mill passes through.

The ball sweeps the pipe about its center's helix, which touches what the side sweeps along the
helix of the ball's widest circle.
