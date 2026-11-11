+++
title = "A thread with a run-out: a bar screwed up a rod, then withdrawn"
tags = ["swept", "solid-sweep", "screw", "slide", "path", "pieces", "prism", "boolean", "cut"]
origin = "geomsolver #721, box K1 (paths made of pieces, a screw among them)"

[volume]
value = 1364.02421385
exact = "no closed form: cad-kernel, OCCT and cadrs agree to 4e-11; the mean of their values"

[area]
value = 822.6335769893
exact = "no closed form: cad-kernel, OCCT and cadrs agree to 3e-11; the mean of their values"
+++

A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a thread cut partway up
it by a bar that is then withdrawn straight out of its groove, as a lathe's tool runs out of a
thread. The tool's path is made of two pieces.

The bar, as placed, is the box 4 ≤ x ≤ 12, −8 ≤ y ≤ 8, −1 ≤ z ≤ 1: it points at the rod's axis, its
flat tip 1 inside the rod, and it is 2 wide along the axis.

First it is carried along a screw about the z axis, turning right-handed (from +x toward +y) and
rising 6 for every whole turn, rigidly, from a turn of −90° (lowered 1.5, wholly below the rod) to
one of +900° (raised 15), where it points along −x, its tip at (−4, 0, 15). Then it slides without
turning along −x by 3, straight back along its own length, until its tip is at (−7, 0, 15), clear of
the rod. Cut from the rod everything the bar passes through.

The two pieces share the bar where the screw stops and the slide starts. The run-out is the bar's
section carried straight out of the groove.
