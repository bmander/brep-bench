+++
title = "A half cylinder with every edge rounded"
tags = ["fillet", "corner", "cylinder", "torus", "sphere", "analytic", "adversarial"]
origin = "adversarial: fillet/box-all-edges with the box a half cylinder: the corners where three rounds meet now have a curved face among their three"

[volume]
value = 376.5487026580339
exact = "h(A + Pr + πr²) + 2(Ar + πPr²/4 + 2πr³/3), A = ρ²·acos(r/ρ) − r√(ρ² − r²), P = 2ρ·acos(r/ρ) + 2√(ρ² − r²), ρ = R − r = 4, h = H − 2r = 8, R = 5, H = 10, r = 1"
tex = 'h\left(A + Pr + \pi r^2\right) + 2\left(Ar + \tfrac{\pi}{4}Pr^2 + \tfrac{2}{3}\pi r^3\right),\quad A = \rho^2\arccos\tfrac{r}{\rho} - r\sqrt{\rho^2 - r^2},\ P = 2\rho\arccos\tfrac{r}{\rho} + 2\sqrt{\rho^2 - r^2},\quad \rho = R - r = 4,\ h = H - 2r = 8,\ r = 1'

[area]
value = 301.05530499147693
exact = "h(P + 2πr) + 2(A + πPr/2 + 2πr²), A, P, ρ, h and r as above"
tex = 'h\left(P + 2\pi r\right) + 2\left(A + \tfrac{\pi}{2}Pr + 2\pi r^2\right)'
+++

The half of the cylinder of radius 5 about the z axis from z = 0 to z = 10 where y ≥ 0: a
half-round bar with a flat face on the plane y = 0. Round every one of its edges to radius 1: the
two half-circle rims where the round face meets the ends, the two straight edges where it meets
the flat face, and the four straight edges where the flat face meets the ends. Three rounds meet
at each of the four corners, closed by the ball of radius 1 that touches all three faces there.

The result is the union of every ball of radius 1 inside the half cylinder: the half cylinder
shrunk by 1 (the circle's segment of radius 4 above the chord y = 1, from z = 1 to z = 9) grown
by 1. Its section at a height inside z = 1..9 is that segment grown by 1, of area A + Pr + πr²,
A and P the segment's area and perimeter. The rims' rounds are pieces of tori about the z axis,
the straight edges' rounds pieces of cylinders, and the corners pieces of balls.

**As a set:**

$$\Bigl(\bigl\{\,\rho \le 5,\ y \ge 0,\ 0 \le z \le 10\,\bigr\} \ominus B_1\Bigr) \oplus B_1$$
