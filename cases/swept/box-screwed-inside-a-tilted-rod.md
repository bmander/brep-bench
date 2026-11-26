+++
title = "A sealed helical tunnel in a tilted rod: a box screwed inside it"
tags = ["swept", "solid-sweep", "screw", "helix", "prism", "void", "boolean", "cut", "adversarial"]
origin = "adversarial: swept/box-screwed-inside-a-rod turned rigidly so that its axis is (1, 2, 2)/3"

[volume]
value = 1522.316218895
exact = "no closed form: swept/box-screwed-inside-a-rod's answer (the mean of cad-kernel's and OCCT's values, which agree to 4e-11), the solid being that one turned rigidly"

[area]
value = 961.1374881545
exact = "no closed form: swept/box-screwed-inside-a-rod's answer (the mean of cad-kernel's and OCCT's values, which agree to 7e-11), the solid being that one turned rigidly"
+++

The rod and tunnel of `swept/box-screwed-inside-a-rod`, turned rigidly about the origin so that the
rod's axis is no longer a coordinate axis. Write u = (2, −2, 1)/3, v = (2, 1, −2)/3 and w = (1, 2, 2)/3: three unit vectors square to one another, right-handed (u × v = w). The point that case puts at (x, y, z) is here at
x·u + y·v + z·w.

So: a rod, the cylinder of radius 5 about the line through the origin along w, from the origin to
20w = (20/3, 40/3, 40/3), with a sealed helical tunnel inside it: everything a small box passes
through as it is carried along a screw wholly within the rod.

The box, as placed, is a cube of side 1 with its edges along u, v and w: every point p with
2.5 ≤ p·u ≤ 3.5, −0.5 ≤ p·v ≤ 0.5 and 2 ≤ p·w ≤ 3. Its corner 2.5u − 0.5v + 2w is (2, −1/2, 5/2).

The screw is about the rod's axis: the box turns right-handed about w (from u toward v) and rises 4
along w for every whole turn, rigidly, from a turn of 0° to one of 720°, ending as the box
2.5 ≤ p·u ≤ 3.5, −0.5 ≤ p·v ≤ 0.5, 10 ≤ p·w ≤ 11. Cut from the rod everything the box passes
through.

The box stays inside the rod all the way, so the result is the rod with a closed helical cavity two
turns long, square in section and ended by the box's own faces: two shells. The box's faces square
to w sweep helicoidal sheets and its other faces cylinders and ruled sheets, meeting along its
edges' helices.
