+++
title = "A rod threaded by a ball"
tags = ["swept", "solid-sweep", "canal", "helix", "boolean", "cut"]
origin = "geomsolver rust/examples/swept_spring.sv"

[volume]
value = 1343.0088854765736
exact = "no closed form (the groove's section is bounded by the tube's cut with the rod): arris and cad-kernel, built independently from this description, agree to 8e-11 (1343.0088855322617 and 1343.0088854208852; the mean); OCCT also agrees, to 2e-9"

[area]
value = 880.5540322252776
exact = "no closed form (the groove's section is bounded by the tube's cut with the rod): arris and cad-kernel, built independently from this description, agree to 6e-11 (880.5540321979142 and 880.554032252641; the mean); OCCT also agrees, to 2e-9"
+++

A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a round-bottomed helical
groove cut round it by a ball of radius 1. The ball's centre follows a helix of radius 5 about the
z axis, so it runs on the rod's surface: it starts at (5, 0, −2), turns right-handed (from +x towards
+y) and rises 4 each turn, for six whole turns, ending at (5, 0, 22). The ball is clear of the rod at
both ends. Cut from the rod everything the ball passes through.

Neighbouring turns of the groove do not meet: they are 4 apart and the ball is 2 across.
