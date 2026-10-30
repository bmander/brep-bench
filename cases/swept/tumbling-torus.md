+++
title = "A torus tumbled through space"
tags = ["swept", "solid-sweep", "tumble", "torus", "envelope"]
origin = "geomsolver rust/examples/swept_tumble.sv"
settled = false
+++

The solid a torus fills as it tumbles: it turns about one axis while being watched from a frame that
turns about another.

The torus has its centre at (3, 0, 2) and its axis parallel to the z axis; the circle through its
tube's centres has radius 1, and its tube has radius 0.5.

It moves by two turns at once, both driven by one parameter t:

- it turns by t/4 about the vertical line through (2, 0, 0), right-handed (from +x towards +y);
- the frame it is seen from turns by t about the line through (0, 0.5, 0.5) parallel to the x axis,
  right-handed (from +y towards +z).

Seen from that frame, the torus's place at t is its place turned by t/4 about the first line and
then by −t about the second. Make the solid of everything the torus passes through, so seen, as t
runs from −75° to 75°.

No turn keeps any line of the torus fixed, so the boundary is an envelope of no simpler kind.
