+++
title = "A square-to-octagon transition: corners lofted to sides"
tags = ["loft", "ruled", "transition", "unequal-sections", "hull"]

[volume]
value = 33760.0
exact = "h(|S| + M + |O|)/3 = 30·(1600 + 1120 + 656)/3: the section at height z = 30t is the Minkowski combination (1 − t)S + tO of the square S and the octagon O, of area (1 − t)²|S| + 2t(1 − t)M + t²|O|, M = ½Σ h_S(n)·ℓ_O(n) = ½(4·20·12 + 4·20√2·8√2) = 1120 their mixed area (h_S the square's support in the direction n of each of the octagon's sides, ℓ_O that side's length)"
tex = '\tfrac{h}{3}\left(|S| + M + |O|\right) = \tfrac{30}{3}(1600 + 1120 + 656),\quad M = \tfrac{1}{2}\left(4\cdot 20\cdot 12 + 4\cdot 20\sqrt{2}\cdot 8\sqrt{2}\right) = 1120'

[area]
value = 6188.2546980536465
exact = "1600 + 656 + 4·½(40 + 12)·√(6² + 30²) + 4·½·8√2·√1100 = 2256 + 624√26 + 160√22: the floor, the top, four trapezoids and four triangles (a triangle's height is the distance from the square's corner to the line of the octagon's side, √(14² + 6² + 30² − (4√2)²))"
tex = '1600 + 656 + 4\cdot\tfrac{40 + 12}{2}\sqrt{6^2 + 30^2} + 4\cdot\tfrac{1}{2}\cdot 8\sqrt{2}\cdot\sqrt{1100} = 2256 + 624\sqrt{26} + 160\sqrt{22}'
+++

A transition piece, as a sheet-metal hood is made, from a square at its foot to an octagon at its
top: a loft between two sections with different numbers of corners.

The square is −20 ≤ x ≤ 20, −20 ≤ y ≤ 20 in the plane z = 0. The octagon lies in the plane
z = 30, centred on the z axis: it is the square 28 × 28 with its corners cut off, its eight
corners (6, −14), (14, −6), (14, 6), (6, 14), (−6, 14), (−14, 6), (−14, −6) and (−6, −14). Its
four sides facing ±x and ±y are 12 long, each parallel to a side of the square below it; its four
cut sides are 8√2 long, each facing a corner of the square.

Each side of the square is joined to the side of the octagon parallel to it by a flat trapezoid:
the square's side from (−20, −20, 0) to (20, −20, 0) to the octagon's from (−6, −14, 30) to
(6, −14, 30), and so on round. Each corner of the square is joined to the cut side of the octagon
facing it by a flat triangle: the corner (20, −20, 0) to the side from (6, −14, 30) to
(14, −6, 30), and so on round. So the loft pairs each of the square's four corners with a whole
side of the octagon, and its eight walls are flat: four trapezoids and four triangles, alternating
round. The solid is the convex hull of the square and the octagon; its floor and top are flat.
