+++
title = "A block under a patch whose folded-back layer lies just beneath it, pierced"
tags = ["nurbs", "bezier", "boolean", "cut", "hole", "trimmed", "fold", "regression"]
origin = "geomsolver #1013: the crossed tumbling tori's Booleans, one capability they lack pinned alone"

[volume]
value = 0.54336907553665484
exact = "|A| − |A ∩ B|: ∫∫ f over 9/100 ≤ x ≤ 1, 0 ≤ y ≤ 1 (0.600826128: 0.6004271 + (1 − 0.09^{5/2})/2500), less ∫∫ f over the hole 2/5 ≤ x ≤ 7/10, 3/10 ≤ y ≤ 3/5 (0.057457052463345157), f = 1/2 + 3x²/10 + x^{3/2}/1000 + y/10"
tex = '\iint_{A} f\,dA - \int_{2/5}^{7/10}\!\!\int_{3/10}^{3/5} f\,dy\,dx,\quad f = \tfrac12 + \tfrac{3x^2}{10} + \tfrac{x^{3/2}}{1000} + \tfrac{y}{10}'

[area]
value = 5.070253321350833
exact = "the top ∫∫√(101/100 + (3x/5 + 3√x/2000)²) over its plan less the hole (by mpmath quadrature), the floor 0.82, and each of the eight walls ∫ of f along it"
tex = '\iint \sqrt{\tfrac{101}{100} + \left(\tfrac{3x}{5} + \tfrac{3\sqrt{x}}{2000}\right)^2}\,dA + 0.82 + \sum_{\text{walls}} \int f\,ds'
+++

A block under a curved top, with a square hole cut straight through it. The top is trimmed on a
Bézier patch that runs on past the face and folds back to a second layer lying just beneath the
face, a few ten-thousandths below it.

The patch is the Bézier surface of degree 4 in u and 1 in v (knot vectors (0, 0, 0, 0, 0, 1, 1, 1,
1, 1) and (0, 0, 1, 1)) whose control points are Pᵢⱼ = (xᵢ, j, zᵢ + j/10), i = 0 … 4, j = 0, 1:

| i | 0 | 1 | 2 | 3 | 4 |
|---|---|---|---|---|---|
| xᵢ | 1 | 0 | −1/3 | 0 | 1 |
| zᵢ | 799/1000 | 401/2000 | 4/5 | 399/2000 | 801/1000 |

It is S(u, v) = (s², v, 1/2 + 3s⁴/10 + s³/1000 + v/10) with s = 2u − 1: over u ≥ 1/2 the graph of
f = 1/2 + 3x²/10 + x^{3/2}/1000 + y/10 over 0 ≤ x ≤ 1, 0 ≤ y ≤ 1; over u ≤ 1/2 a second layer
x^{3/2}/500 below it, the two meeting in a cuspidal edge along x = 0.

The block A stands on the floor z = 0 under the part of the patch with 13/20 ≤ u ≤ 1: the region
9/100 ≤ x ≤ 1, 0 ≤ y ≤ 1, up to the patch. Its top face lies on the patch (the whole patch, trimmed
to 13/20 ≤ u ≤ 1); its other five faces are planes.

The solid is A less the box 2/5 ≤ x ≤ 7/10, 3/10 ≤ y ≤ 3/5, −1 ≤ z ≤ 3: a square hole straight
through the block. Each upright edge of the hole crosses the top face once, and passes the patch's
second layer between 1.8e-4 and 5.7e-4 below it.

A kernel may build the top face on the patch cut down to its part (13/20 ≤ u ≤ 1), which is the same
face. The case is about a face whose surface has a second layer close beside it: geomsolver's
crossed tumbling tori have faces trimmed on fitted nets that fold back past the faces' cuspidal
edges, the folded layer within a few ten-thousandths of the face near the edge (3.5e-4 apart where
they were measured).

**As a set:**

$$\{\, 9/100 \le x \le 1,\ 0 \le y \le 1,\ 0 \le z \le \tfrac12 + \tfrac{3x^2}{10} + \tfrac{x^{3/2}}{1000} + \tfrac{y}{10} \,\} \setminus [\tfrac25, \tfrac7{10}] \times [\tfrac3{10}, \tfrac35] \times [-1, 3]$$
