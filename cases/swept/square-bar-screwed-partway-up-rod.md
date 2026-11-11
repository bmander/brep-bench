+++
title = "A thread stopped short: a bar screwed partway up a rod"
tags = ["swept", "solid-sweep", "screw", "helix", "prism", "stopped", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/stopped_screw.sv"

[volume]
value = 1431.585742257
exact = "no closed form: cad-kernel, OCCT and cadrs agree to 3e-11; the mean of their values"

[area]
value = 806.9351761057
exact = "no closed form: cad-kernel, OCCT and cadrs agree to 4e-11; the mean of their values"
+++

A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a thread cut partway up
it by a bar carried along a screw, stopping inside the rod as a lathe's thread stops short of a
shoulder.

The bar, as placed, is the box 4 ≤ x ≤ 12, −8 ≤ y ≤ 8, −1 ≤ z ≤ 1: it points at the rod's axis, its
flat tip (the face x = 4) 1 inside the rod, and it is 2 wide along the axis.

The screw is about the z axis: the bar turns right-handed about it (from +x toward +y) and rises 6
for every whole turn, rigidly. It runs from a turn of −120° (the bar lowered 2, wholly below the
rod) to a turn of +600° (raised 10), where it stops with its middle halfway up the rod. Cut from the
rod everything the bar passes through.

The groove is `square-bar-screwed-through-rod`'s up to where the bar stops, and ends there in the
bar's own faces.
