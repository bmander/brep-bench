+++
title = "A curved block grooved by a leaning cutter turned past it"
tags = ["swept", "solid-sweep", "turn", "torus", "envelope", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/turn.sv"
settled = false
+++

A curved block, grooved by a cutter turned past it about the block's own axis.

The block is a sector of a ring about the z axis: everything with distance from the z axis between 8
and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x towards
+y).

The cutter is a torus whose axis passes through (5, 0, 0), leaning from the z axis towards +y by 25°:
its direction is (0, sin 25°, cos 25°). Its centre is (5, 0, 0); the circle through its tube's
centres has radius 2.5, and its tube has radius 2. So, as placed, its tube reaches out to x = 9.5
along the x axis, into the block's inner side, but the cutter is turned out of the block before it
starts.

The cutter turns rigidly about the z axis, right-handed (from +x towards +y), from −140° to +140°:
it starts clear of the block, passes through it, and ends clear of it. Cut from the block everything
the cutter passes through.

Leaning across the turn, the cutter's axis is skew to the block's, so the groove's wall is none of
the torus's own circles.
