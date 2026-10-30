+++
title = "A box slid aslant"
tags = ["swept", "solid-sweep", "slide", "polyhedron", "zonotope"]

[volume]
value = 812.0
exact = "abc + |dx|bc + |dy|ca + |dz|ab, a, b, c = 10, 6, 4, d = (8, 5, 3)"

[area]
value = 542.6189318971692
exact = "2(ab + bc + ca) + 2(a|e₁×d| + b|e₂×d| + c|e₃×d|)"
+++

The solid a box fills as it slides along a straight line without turning. The box starts as the box
from (0, 0, 0) to (10, 6, 4) and slides along the vector (8, 5, 3), ending as the box from (8, 5, 3)
to (18, 11, 7). Make the solid of everything the box passes through.

It is a convex polyhedron with twelve faces, in six parallel pairs: the box's three pairs of faces,
and three pairs of parallelograms, each with one edge of the box and the slide's vector as its sides.
