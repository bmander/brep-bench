+++
title = "A ball screwed round a helix tighter than itself, about a tilted axis"
tags = ["swept", "solid-sweep", "screw", "canal", "helix", "fold", "self-intersecting", "adversarial"]
origin = "adversarial: swept/ball-screwed-round-a-tight-helix turned rigidly so that its axis is (1, 2, 2)/3"

[volume]
value = 253.449373663
exact = "no closed form: swept/ball-screwed-round-a-tight-helix's answer (the mean of cad-kernel's and cadrs's values, which agree to 5e-10), the solid being that one turned rigidly"

[area]
value = 220.3147392925
exact = "no closed form: swept/ball-screwed-round-a-tight-helix's answer (the mean of cad-kernel's and cadrs's values, which agree to 3e-10), the solid being that one turned rigidly"
+++

The solid of `swept/ball-screwed-round-a-tight-helix`, turned rigidly about the origin so that its
axis is no longer a coordinate axis. Write u = (2, −2, 1)/3, v = (2, 1, −2)/3 and w = (1, 2, 2)/3: three unit vectors square to one another, right-handed (u × v = w). The point that case puts at (x, y, z) is here at
x·u + y·v + z·w.

So: a ball of radius 2 whose center runs along a helix about the line through the origin along w.
The center starts at 1.5u = (1, −1, 1/2) and runs along the helix of radius 1.5 about that line
that turns right-handed about w (from u toward v) and rises 3 along w every turn, for two whole
turns, ending at 1.5u + 6w = (3, 3, 9/2). Make the solid of everything the ball passes through.

The helix's radius of curvature, (1.5² + (3/2π)²)/1.5 ≈ 1.65, is less than the ball's radius, so the
envelope of the balls folds: its inner side is cut away by the balls themselves. And the turns, 3
apart, overlap. The solid is everything within 2 of the helix's arc.
