+++
title = "A rod grooved by a wheel screwed along it"
tags = ["swept", "solid-sweep", "screw", "helix", "torus", "envelope", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/screw.sv"
settled = false
+++

A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a helical groove ground
round it by a wheel carried along a screw, as a thread is ground.

The wheel is a torus: the circle through its tube's centres has radius 6, and its tube has radius 2.
As placed, its centre is (11.5, 0, 0), so the part of its tube nearest the rod is centred at
(5.5, 0, 0) and reaches 1.5 into the rod. Its axis passes through its centre square to the x axis,
leaning from the y axis down towards −z by the helix angle h = atan(2π·5.5 / 30) ≈ 49.04°: the axis's
direction is (0, cos h, −sin h). The wheel then lies along the helix that the screw below carries the
point (5.5, 0, 0) along, as a grinding wheel is set.

The screw is about the z axis: the wheel turns right-handed about it (from +x towards +y) and rises
30 for every whole turn, rigidly. It runs from a turn of −108° (the wheel lowered 9, wholly below the
rod) to a turn of +348° (raised 29, wholly above it). Cut from the rod everything the wheel passes
through.

The groove's section square to the axis is the same at every height, turned.
