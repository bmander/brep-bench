+++
title = "A block under a Bézier patch of degree 15"
tags = ["geometry", "nurbs", "bezier", "high-degree", "surface"]

[volume]
value = 2795.3125
exact = "400·Σzᵢⱼ/256 = 400·1789/256 = 44725/16: x = 20u and y = 20v, and each Bernstein polynomial of degree 15 integrates to 1/16 over [0, 1]"
tex = '400\cdot\tfrac{1}{256}\sum_{i,j} z_{ij} = \tfrac{44725}{16}'

[area]
value = 1358.7504625329134
exact = "no closed form (the patch's area element is the root of a polynomial): OCCT (1358.75046031368) and cad-kernel (1358.7504647521466), built independently from this description, agree to 3e-9; the mean"
+++

A block standing on the square 0 ≤ x ≤ 20, 0 ≤ y ≤ 20 of the xy plane, its floor flat at z = 0,
its four sides upright in the planes x = 0, x = 20, y = 0 and y = 20, and its top a Bézier patch of
degree 15 each way, non-rational: a net of 16 × 16 control points.

The patch is S(u, v) = Σᵢ Σⱼ Bᵢ(u) Bⱼ(v) Pᵢⱼ for u, v from 0 to 1 and i, j = 0 … 15, with the
Bernstein polynomials of degree 15, Bᵢ(t) = C(15, i) tⁱ (1 − t)¹⁵⁻ⁱ. The control point Pᵢⱼ is
(20i/15, 20j/15, zᵢⱼ), standing over an even 16 × 16 grid of the square, at the height
zᵢⱼ = 4 + ((3i + 5j) mod 7): a field of heights from 4 to 10, never the same at two neighbours.

Because the control points' x and y lie on an even grid, x = 20u and y = 20v, so the patch is the
graph of a height over the square, everywhere at least 4 above the floor, and its four edges are
the upright sides' top edges: each side is the flat region between the floor and the curve of
degree 15 the patch's edge draws in that side's plane.
