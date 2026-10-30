+++
title = "A rod threaded by a V wheel"
tags = ["swept", "solid-sweep", "screw", "helix", "envelope", "sharp-edge", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/vthread.sv"
settled = false
+++

A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a helical V groove ground
round it by a wheel with a sharp V rim, carried along a screw, as a thread is ground.

The wheel's axis passes through (14, 0, 0), square to the x axis, leaning from the y axis down
towards −z by the helix angle h = atan(2π·4.5 / 30) ≈ 43.30°: its direction is (0, cos h, −sin h).
Call the direction (1, 0, 0) "out" and the axis's direction "along". The wheel is the solid of
revolution, about this axis, of a triangle in the plane through the axis and the point (4, 0, 0):

- its tip is at (4, 0, 0), 10 from the axis on the rod's side, and 1 inside the rod;
- its other two corners are 2.5 further from the rod, at (4, 0, 0) + 2.5·out ± 2.5·tan 30°·along.

So the wheel is a ring 2.89 thick at its bore (radius 7.5) whose rim narrows to a sharp edge of
radius 10, each flank 30° from the wheel's mid-plane (a 60° V).

The screw is about the z axis: the wheel turns right-handed about it (from +x towards +y) and rises
30 for every whole turn, rigidly. It runs from a turn of −144° (the wheel lowered 12, wholly below the
rod) to a turn of +384° (raised 32, wholly above it). Cut from the rod everything the wheel passes
through.
