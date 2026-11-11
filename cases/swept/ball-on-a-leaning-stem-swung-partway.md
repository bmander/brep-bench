+++
title = "A ball on a leaning stem swung partway into a block"
tags = ["swept", "solid-sweep", "turn", "sphere", "cylinder", "concave", "stopped", "boolean", "cut"]
origin = "geomsolver #721, box X9 (the ball on a leaning stem turned part of a turn)"

[volume]
value = 71.03324855515
exact = "no closed form: cad-kernel and cadrs agree to 2e-11; the mean of their values"

[area]
value = 164.1729535245
exact = "no closed form: cad-kernel and cadrs agree to 7e-12; the mean of their values"
+++

A curved block with a notch cut into it by a ball on a leaning stem, swung partway into it.

The block is a sector of a ring about the z axis: everything with distance from the z axis between 8
and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The tool is the union of a ball of radius 0.8 centered at (10, 0, 0) and a stem, a solid cylinder of
radius 0.4 whose axis runs from the ball's center 3 along the direction (0, sin 20°, cos 20°),
leaning 20° from upright toward +y, its far end flat. The stem stands out through the block's top.
Where the stem leaves the ball the tool has a concave seam.

It turns rigidly about the z axis, right-handed (from +x toward +y), from −140° to +30°: it starts
clear of the block and stops halfway across it. Cut from the block everything the tool passes
through.
