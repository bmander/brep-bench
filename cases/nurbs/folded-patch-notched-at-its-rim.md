+++
title = "A block under a patch folded over its own rim, notched across the rim"
tags = ["nurbs", "bezier", "boolean", "cut", "trimmed", "fold", "regression"]
origin = "geomsolver #1013: the crossed tumbling tori's Booleans, one capability they lack pinned alone"

[volume]
value = 0.48658855
exact = "|A| − |A ∩ B| = 0.6004271 − 0.11383855: ∫∫(1/2 + 3x²/10 + y/10) over 9/100 ≤ x ≤ 1, 0 ≤ y ≤ 1, less the same over 9/100 ≤ x ≤ 1/2, 0 ≤ y ≤ 1/2"
tex = '\int_{9/100}^{1}\!\!\int_0^1 f\,dy\,dx - \int_{9/100}^{1/2}\!\!\int_0^{1/2} f\,dy\,dx,\quad f = \tfrac12 + \tfrac{3x^2}{10} + \tfrac{y}{10}'

[area]
value = 4.1267385566944163
exact = "the top ∫∫√(101/100 + 9x²/25) over its L-shaped plan (∫√(a + b²x²)dx = (x/2)√(a + b²x²) + (a/2b) arsinh(bx/√a)), the floor 0.705, and each of the eight walls ∫ of the height f along it"
tex = '\iint_L \sqrt{\tfrac{101}{100} + \tfrac{9x^2}{25}}\,dA + 0.705 + \sum_{\text{walls}} \int f\,ds'
+++

A block under a curved top, with a corner notched out of it by a box. The top is the interesting
face: it is trimmed on a Bézier patch that runs on past the face, folds back over itself, and
covers the face a second time.

The patch is the Bézier surface of degree 4 in u and 1 in v (knot vectors (0, 0, 0, 0, 0, 1, 1, 1,
1, 1) and (0, 0, 1, 1)) whose control points are Pᵢⱼ = (xᵢ, j, zᵢ + j/10), i = 0 … 4, j = 0, 1:

| i | 0 | 1 | 2 | 3 | 4 |
|---|---|---|---|---|---|
| xᵢ | 1 | 0 | −1/3 | 0 | 1 |
| zᵢ | 4/5 | 1/5 | 4/5 | 1/5 | 4/5 |

It is S(u, v) = (s², v, 1/2 + 3s⁴/10 + v/10) with s = 2u − 1: over u ≥ 1/2 it is the graph of
z = 1/2 + 3x²/10 + y/10 over 0 ≤ x ≤ 1, 0 ≤ y ≤ 1, and over u ≤ 1/2 it is the same graph again,
the patch folding back along u = 1/2 (the line x = 0).

The block A stands on the floor z = 0 under the part of the patch with 13/20 ≤ u ≤ 1: the region
9/100 ≤ x ≤ 1, 0 ≤ y ≤ 1, up to the patch. Its top face lies on the patch (the whole patch, trimmed
to 13/20 ≤ u ≤ 1, whose edge u = 13/20 is the top's edge over the wall x = 9/100); its other five
faces are planes.

The solid is A less the box 1/20 ≤ x ≤ 1/2, −1 ≤ y ≤ 1/2, −1 ≤ z ≤ 3, which notches out the corner
of A at x = 9/100, y = 0. The box's side y = 1/2 crosses the top face's edge u = 13/20, where the
patch's folded-back part u ≤ 7/20 covers the face again.

A kernel may build the top face on the patch cut down to its part (13/20 ≤ u ≤ 1), which is the same
face. The case is about a face whose surface covers the face's own neighbourhood twice: geomsolver's
crossed tumbling tori (swept/tumbling-torus-crossed-union and the others) have faces trimmed on
fitted nets that fold back past the faces' cuspidal edges.

**As a set:**

$$\{\, 9/100 \le x \le 1,\ 0 \le y \le 1,\ 0 \le z \le \tfrac12 + \tfrac{3x^2}{10} + \tfrac{y}{10} \,\} \setminus [\tfrac1{20}, \tfrac12] \times [-1, \tfrac12] \times [-1, 3]$$
