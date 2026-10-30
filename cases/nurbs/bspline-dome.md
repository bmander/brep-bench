+++
title = "A block under a bicubic B-spline with a double knot"
tags = ["nurbs", "bspline", "knots", "surface"]

[volume]
value = 3081.0055555555555
exact = "Σᵢⱼ zᵢⱼ·aᵢ·bⱼ = 554581/180, aᵢ = ∫Nᵢ(u)x′(u)du (119, 117, 231, 224, 153, 156)/50, bⱼ = ∫Mⱼ(v)y′(v)dv (105, 187, 151, 157, 120)/36, exact piecewise-polynomial integrals"

[area]
value = 1285.368404802979
exact = "floor 400 + the four upright sides 3421/36 + 2701/36 + 4301/50 + 4647/50 (∫ height dy along each edge, exactly) = 749.0155…, + the curved top, which has no closed form: OCCT's measure of its top face (536.3528491467879) and cad-kernel's (536.3528493480587, its total less the closed-form faces) agree to 4e-10; the mean. OCCT's own total is off (its integration of the knotted sides is), cad-kernel's whole solid agrees with this to 1e-10"
+++

A block standing on the square 0 ≤ x ≤ 20, 0 ≤ y ≤ 20 of the xy plane, its floor flat at z = 0,
its four sides upright in the planes x = 0, x = 20, y = 0 and y = 20, and its top a clamped
bicubic B-spline surface, non-rational, of 6 × 5 control points.

The surface is S(u, v) = Σᵢ Σⱼ Nᵢ(u) Mⱼ(v) Pᵢⱼ for u, v from 0 to 1, where Nᵢ (i = 0 … 5) are the
cubic B-spline basis functions on the knot vector (0, 0, 0, 0, 2/5, 2/5, 1, 1, 1, 1), with a
double knot at 2/5, and Mⱼ (j = 0 … 4) the cubic ones on (0, 0, 0, 0, 1/3, 1, 1, 1, 1). The control
point Pᵢⱼ = (xᵢ, yⱼ, zᵢⱼ) has x₀ … x₅ = 0, 3, 7, 11, 16, 20 and y₀ … y₄ = 0, 5, 10, 15, 20, and
these heights zᵢⱼ:

| | j = 0 | j = 1 | j = 2 | j = 3 | j = 4 |
|---|---|---|---|---|---|
| i = 0 | 3 | 5 | 6 | 5 | 4 |
| i = 1 | 5 | 9 | 11 | 8 | 5 |
| i = 2 | 6 | 12 | 15 | 10 | 6 |
| i = 3 | 4 | 10 | 18 | 9 | 5 |
| i = 4 | 5 | 8 | 9 | 7 | 4 |
| i = 5 | 2 | 4 | 5 | 4 | 3 |

Because every control point of row i has the same x and every one of column j the same y, the
surface's x depends on u alone and its y on v alone, each rising steadily from 0 to 20 (but not
in proportion to u or v: the control points are not at the knots' averages). So the surface is
the graph of a height over the square, everywhere at least 2 above the floor, and its four edges
lie in the four sides' planes: each side is the flat region between the floor and the cubic
B-spline curve the surface's edge draws in that plane. The double knot leaves the surface only
C¹ across u = 2/5: its curvature jumps along that line.
