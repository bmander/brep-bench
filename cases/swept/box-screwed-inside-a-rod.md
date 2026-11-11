+++
title = "A sealed helical tunnel: a box screwed inside a rod"
tags = ["swept", "solid-sweep", "screw", "helix", "prism", "void", "boolean", "cut"]
origin = "geomsolver #721, box D4 (a prism screwed wholly within)"

[volume]
value = 1522.316218895
exact = "no closed form: cad-kernel and OCCT agree to 4e-11; the mean of their values (cadrs is 1.2e-8 from it)"

[area]
value = 961.1374881545
exact = "no closed form: cad-kernel and OCCT agree to 7e-11; the mean of their values (cadrs is 1.2e-8 from it)"
+++

A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a sealed helical tunnel
inside it: everything a small box passes through as it is carried along a screw wholly within the
rod.

The box, as placed, is 2.5 ≤ x ≤ 3.5, −0.5 ≤ y ≤ 0.5, 2 ≤ z ≤ 3.

The screw is about the z axis: the box turns right-handed about it (from +x toward +y) and rises 4
for every whole turn, rigidly, from a turn of 0° to one of 720°, ending as the box 2.5 ≤ x ≤ 3.5,
−0.5 ≤ y ≤ 0.5, 10 ≤ z ≤ 11. Cut from the rod everything the box passes through.

The box stays inside the rod all the way, so the result is the rod with a closed helical cavity two
turns long, square in section and ended by the box's own faces: two shells. The box's level faces
sweep helicoidal sheets and its upright faces cylinders and ruled sheets, meeting along its edges'
helices.
