+++
title = "A ball screwed round a helix tighter than itself, ten times larger"
tags = ["swept", "solid-sweep", "screw", "canal", "helix", "fold", "self-intersecting", "large", "scale", "adversarial"]
origin = "adversarial: swept/ball-screwed-round-a-tight-helix ten times larger"

[volume]
value = 253449.373663
exact = "no closed form: 10³ times swept/ball-screwed-round-a-tight-helix's answer (the mean of cad-kernel's and cadrs's values, which agree to 5e-10), the solid being that one scaled by 10 about the origin"

[area]
value = 22031.47392925
exact = "no closed form: 10² times swept/ball-screwed-round-a-tight-helix's answer (the mean of cad-kernel's and cadrs's values, which agree to 3e-10), the solid being that one scaled by 10 about the origin"
+++

The solid of `swept/ball-screwed-round-a-tight-helix`, ten times larger: the solid a ball fills as
its center runs along a helix whose curvature is tighter than the ball, so that the tube about the
helix folds through itself.

The ball has radius 20. Its center starts at (15, 0, 0) and runs along the helix of radius 15
about the z axis that turns right-handed (from +x toward +y) and rises 30 every turn, for two whole
turns, ending at (15, 0, 60). Make the solid of everything the ball passes through.

The helix's radius of curvature, (15² + (30/2π)²)/15 ≈ 16.5, is less than the ball's radius, so the
envelope of the balls folds: its inner side is cut away by the balls themselves. And the turns, 30
apart, overlap. The solid is everything within 20 of the helix's arc.
