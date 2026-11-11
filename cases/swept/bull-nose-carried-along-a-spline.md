+++
title = "A leaning bull-nose mill carried along a spline"
tags = ["swept", "solid-sweep", "frame", "general-motion", "spline", "torus", "leaning", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/spline_mill.sv"

[volume]
value = 10507.5864133
exact = "no closed form: cad-kernel and cadrs agree to 4e-10; the mean of their values"

[area]
value = 3968.62573204
exact = "no closed form: cad-kernel and cadrs agree to 1e-9; the mean of their values"
+++

A block, the box from (−6, −6, −8) to (36, 26, 0), with a groove cut into its top by a leaning
bull-nose mill carried along a spline.

The curve is the cubic Bézier curve in the plane z = 0 whose control points are (0, 0, 0), (10, 0,
0), (20, 20, 0) and (30, 20, 0), run from its first end to its last.

A frame rides along the curve: at each point its origin is the curve's point, its x axis the curve's
tangent there (pointing the way the curve runs), its z axis straight up and its y axis to the left,
so that at the curve's start it is the world's own frame. The tool is drawn in that starting frame
and carried rigidly by the frame as it runs the whole curve.

The mill, so drawn, is a solid of revolution about an axis leaning 30° from upright toward +x (the
way the curve sets out), along a = (sin 30°, 0, cos 30°), through c = (0, 0, −1), the middle of its
flat end, 1 below the block's top. Along its axis it runs from c up to c + 3a: a cylinder of radius
2 whose end corner is rounded to radius 0.5 (its flat end a disc of radius 1.5 about c, its side of
radius 2 from c + 0.5a to c + 3a, and between them a quarter of a torus of tube radius 0.5 centered
on the circle of radius 1.5 about c + 0.5a).

Cut from the block everything the mill passes through as the frame runs the whole curve.

The frame turns the mill as the curve turns, so no screw carries it: the curve along which the mill
touches what it sweeps moves on the mill from instant to instant.
