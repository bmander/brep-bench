+++
title = "A square duct turned to a diamond"
tags = ["sweep", "loft", "ruled", "twisted"]
origin = "geomsolver rust/examples/solid_twist.sv"

[volume]
value = 12373.333333333334
exact = "40·(A₀ + 4A½ + A₁)/6 = 40·(400 + 4·292 + 288)/6 = 37120/3 (each section's area is quadratic in y)"
tex = '\tfrac{40}{6}\left(A_0 + 4A_{1/2} + A_1\right) = \tfrac{40}{6}(400 + 4\cdot 292 + 288) = \tfrac{37120}{3}'

[area]
value = 3515.670710036986
exact = "no closed form (twisted ruled faces): OCCT and arris, built independently from this description, agree to 7e-13"
+++

A solid duct running along the y axis from y = 0 to y = 40, with a square at its start and a
diamond at its end, each side a twisted ruled face.

At y = 0 the section is the square with corners A (10, 0, −10), B (−10, 0, −10), C (−10, 0, 10)
and D (10, 0, 10). At y = 40 it is the diamond with corners A′ (0, 40, −12), B′ (−12, 40, 0),
C′ (0, 40, 12) and D′ (12, 40, 0): its corners are the square's turned an eighth of a turn about
the y axis and pushed out from 10√2 to 12. Each corner of the square runs in a straight line to
the corner of the diamond with the same letter (A to A′, B to B′, C to C′, D to D′), and at every
y between the section is the quadrilateral through the four points those lines reach there.

So each of the four long faces is the ruled surface swept by the straight segment joining two
neighbouring corner lines at the same y: the face through A, B, B′ and A′ is made of the
segments from A + t(A′ − A) to B + t(B′ − B) for t from 0 to 1, and so on round. Since the
square's sides are not parallel to the diamond's, these faces are not flat. The two ends are flat.
