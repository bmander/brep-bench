+++
title = "A block under a cuspidal patch, sliced by a plane through the cusp beside it"
tags = ["nurbs", "bezier", "boolean", "cut", "plane", "trimmed", "cusp", "regression"]
origin = "geomsolver #1013: the crossed tumbling tori's Booleans, one capability they lack pinned alone"

[volume]
value = 0.51031272172170669
exact = "∫ over 1/100 ≤ x ≤ 1 of the column under min(f, 11y/10), f = 1/2 + x^{3/2} + y/10, the two meeting at y* = 1/2 + x^{3/2}: (11/20)y*² + f(x, y*)(1 − y*) + (1 − y*²)/20 for x < 2^{−2/3}, else 11/20 (by mpmath quadrature)"
tex = '\int_{1/100}^{1} \int_0^1 \min\!\left(\tfrac12 + x^{3/2} + \tfrac{y}{10},\ \tfrac{11y}{10}\right) dy\,dx'

[area]
value = 4.2949491490074952
exact = "the top: the plane's part √(221)/10 times its plan, and the patch's ∫∫√(101/100 + 9x/4) over the rest, the floor 0.99, the walls x = 1/100, x = 1 and y = 1 each ∫ of the height along it (by mpmath quadrature)"
+++

A block under a curved top, sliced by a plane. The top is trimmed on a Bézier patch that runs on
past the face to a cusp, a hundredth beyond the face's edge, and back out on its other side; the
plane passes through the cusp.

The patch is the Bézier surface of degree 3 in u and 1 in v (knot vectors (0, 0, 0, 0, 1, 1, 1, 1)
and (0, 0, 1, 1)) whose control points are Pᵢⱼ = (xᵢ, j, zᵢ + j/10), i = 0 … 3, j = 0, 1:

| i | 0 | 1 | 2 | 3 |
|---|---|---|---|---|
| xᵢ | 1 | −1/3 | −1/3 | 1 |
| zᵢ | −1/2 | 3/2 | −1/2 | 3/2 |

It is S(u, v) = (s², v, 1/2 + s³ + v/10) with s = 2u − 1: over u ≥ 1/2 the graph of
f = 1/2 + x^{3/2} + y/10 over 0 ≤ x ≤ 1, 0 ≤ y ≤ 1; over u ≤ 1/2 the graph of 1/2 − x^{3/2} + y/10,
the two meeting in a cusp along u = 1/2, the line (0, y, 1/2 + y/10).

The block A stands on the floor z = 0 under the part of the patch with 11/20 ≤ u ≤ 1: the region
1/100 ≤ x ≤ 1, 0 ≤ y ≤ 1, up to the patch. Its top face lies on the patch (the whole patch, trimmed
to 11/20 ≤ u ≤ 1); its other five faces are planes.

The solid is the part of A on or below the plane z = 11y/10. The plane meets the top face in the
curve y = 1/2 + x^{3/2}, from the face's edge x = 1/100 out to the wall y = 1 near x = 0.63, and runs
on through the patch's cusp at (0, 1/2, 11/20), a hundredth past the face's edge, where its meeting
with the patch turns back along the other layer, y = 1/2 − x^{3/2}.

A kernel may build the top face on the patch cut down to its part (11/20 ≤ u ≤ 1), which is the same
face. The case is about a face whose surface has a cusp just outside it, which a meeting with the
face runs into: geomsolver's crossed tumbling tori have faces trimmed on fitted nets that run on
past the faces' cuspidal edges.

**As a set:**

$$\{\, 1/100 \le x \le 1,\ 0 \le y \le 1,\ 0 \le z \le \min\!\left(\tfrac12 + x^{3/2} + \tfrac{y}{10},\ \tfrac{11y}{10}\right) \,\}$$
