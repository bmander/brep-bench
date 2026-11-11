+++
title = "A square key carried along a spline"
tags = ["swept", "solid-sweep", "frame", "general-motion", "spline", "prism", "boolean", "cut"]
origin = "geomsolver #721, box K2 and X11 (any rigid motion, past convex tools of revolution)"

[volume]
value = 10652.8799891
exact = "no closed form: OCCT and cadrs give the same volume to the twelve figures the board publishes"

[area]
value = 3996.725031465
exact = "no closed form: OCCT and cadrs agree to 2e-11; the mean of their values"
+++

A block, the box from (−6, −6, −8) to (36, 26, 0), with a groove cut into its top by a square key
carried along a spline.

The curve is the cubic Bézier curve in the plane z = 0 whose control points are (0, 0, 0), (10, 0,
0), (20, 20, 0) and (30, 20, 0), run from its first end to its last.

A frame rides along the curve: at each point its origin is the curve's point, its x axis the curve's
tangent there (pointing the way the curve runs), its z axis straight up and its y axis to the left,
so that at the curve's start it is the world's own frame. The tool is drawn in that starting frame
and carried rigidly by the frame as it runs the whole curve.

The key, so drawn, is the box from (−1.5, −0.8, −1.5) to (1.5, 0.8, 2.5): 3 long along the curve,
1.6 across it, its bottom 1.5 below the block's top.

Cut from the block everything the key passes through as the frame runs the whole curve.

The key turns as the curve turns, so no screw carries it, and it is no solid of revolution: its
contact changes from face to face and edge to edge along the way, and its corners dig past its sides
where the curve bends.
