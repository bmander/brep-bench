+++
title = "A floor cut by a leaning bar's flat end, turned"
tags = ["swept", "solid-sweep", "turn", "prism", "cap", "cone", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/flat_end.sv"

[volume]
value = 53.56455344653
exact = "no closed form: cad-kernel, OCCT and cadrs agree to 2e-12; the mean of their values"

[area]
value = 131.121294585
exact = "no closed form: cad-kernel, OCCT and cadrs give the same area to the twelve figures the board publishes"
+++

A curved block whose floor is cut by the flat end of a leaning bar turned past it.

The block is a sector of a ring about the z axis: everything with distance from the z axis between 8
and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The bar is drawn in the plane through (10, 0, 0) spanned by u = (cos 20°, 0, −sin 20°) and v = (0,
1, 0): a point (a, b) of that plane is (10, 0, 0) + a u + b v. Its end is the rectangle 3 by 4 there
turned 5° in that plane, {(a, b): 0 ≤ a cos 5° + b sin 5° ≤ 3, |b cos 5° − a sin 5°| ≤ 2}, whose
side through (10, 0, 0) is the bar's inner side. The bar is that rectangle carried 3 along n = (sin
20°, 0, cos 20°), square to the plane: it leans 20° out from upright, its flat end facing down and
toward the z axis.

It turns rigidly about the z axis, right-handed (from +x toward +y), from −140° to +140°: it starts
clear of the block, passes through it, and ends clear of it. Cut from the block everything the bar
passes through.

The end touches along the line where it meets the plane through the z axis and its normal, which the
turn carries round into the cut's floor, a cone; the inner side touches along a line crossing its
rulings; the rim between them carries the contact from the one to the other. The section is turned
5° so that no side faces the axis squarely.
