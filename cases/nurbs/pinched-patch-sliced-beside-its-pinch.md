+++
title = "A block under a patch pinched to a point beside it, sliced by a plane past the pinch"
tags = ["nurbs", "bezier", "boolean", "cut", "plane", "trimmed", "singular", "regression"]
origin = "geomsolver #1013: the crossed tumbling tori's Booleans, one capability they lack pinned alone"

[volume]
value = 0.5244906492156999
exact = "∫∫ min(f, 10(13/250 + x − y))⁺ over 1/1000 ≤ x, y ≤ 1, f = 1/2 + ∛(xy) (by mpmath quadrature, split where the plane meets the patch and the floor)"
tex = '\int_{1/1000}^{1}\!\!\int_{1/1000}^{1} \max\!\left(0,\ \min\!\left(\tfrac12 + \sqrt[3]{xy},\ 10\left(\tfrac{13}{250} + x - y\right)\right)\right) dy\,dx'

[area]
value = 4.591931211856204
exact = "the patch's ∫∫√(1 + (∛(xy)/3x)² + (∛(xy)/3y)²) where it lies below the plane, the plane's √201 times its plan, the floor where the plane stands above it, and the four walls ∫ of the height along each (by mpmath quadrature)"
+++

A block under a curved top, sliced by a plane. The top is trimmed on a Bézier patch that pinches
to a point just outside the face: its parametrization is singular there alone, and the plane passes
0.002 from it.

The patch is the bicubic Bézier surface (knot vectors (0, 0, 0, 0, 1, 1, 1, 1) both ways) whose
control points are Pᵢⱼ = (xᵢ, xⱼ, 1/2 + sᵢsⱼ), i, j = 0 … 3, with

| i | 0 | 1 | 2 | 3 |
|---|---|---|---|---|
| xᵢ | −1 | 1 | −1 | 1 |
| sᵢ | −1 | −1/3 | 1/3 | 1 |

It is S(u, v) = (s³, t³, 1/2 + st) with s = 2u − 1, t = 2v − 1: the graph of z = 1/2 + ∛(xy) over
−1 ≤ x, y ≤ 1, whose parametrization is singular (S_u × S_v = 0) at u = v = 1/2 alone, the point
(0, 0, 1/2).

The block A stands on the floor z = 0 under the part of the patch with 11/20 ≤ u ≤ 1 and
11/20 ≤ v ≤ 1: the region 1/1000 ≤ x ≤ 1, 1/1000 ≤ y ≤ 1, up to the patch. Its top face lies on the
patch (the whole patch, trimmed to that square of its parameters); its other five faces are planes.

The solid is the part of A where −x + y + z/10 ≤ 13/250: A cut by that plane, which crosses the top
face from near its corner at the pinch out to the wall y = 1, and passes 0.002 from the patch's
singular point (0, 0, 1/2), outside the face.

A kernel may build the top face on the patch cut down to its part, which is the same face. The case
is about a face whose surface has an isolated singular point just outside it, past which a meeting
with the face runs on: geomsolver's crossed tumbling tori have faces trimmed on fitted nets with
singular points just outside the faces.

**As a set:**

$$\{\, 1/1000 \le x \le 1,\ 1/1000 \le y \le 1,\ 0 \le z \le \tfrac12 + \sqrt[3]{xy},\ -x + y + \tfrac{z}{10} \le \tfrac{13}{250} \,\}$$
