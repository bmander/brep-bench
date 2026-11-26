+++
title = "A pin a tenth of a millimetre across, its rim rounded unequally by a quarter ellipse"
tags = ["fillet", "asymmetric", "ellipse", "cylinder", "curved-edge", "revolution", "small", "scale", "adversarial"]
origin = "adversarial: fillet/elliptic-section-cylinder-rim at a hundredth of its size"

[volume]
value = 0.0007660759734642396
exact = "πR²H − 2π[Rab(1 − π/4) − a²b(5/6 − π/4)], R = 0.05, H = 0.1, a = 0.01 across the top, b = 0.03 down the side: the section's area ab(1 − π/4) and its moment about the side a²b(5/6 − π/4)"
tex = '\pi R^2 H - 2\pi\Bigl[Rab\left(1 - \tfrac{\pi}{4}\right) - a^2 b\left(\tfrac{5}{6} - \tfrac{\pi}{4}\right)\Bigr],\quad R = 0.05,\ H = 0.1,\ a = 0.01,\ b = 0.03'

[area]
value = 0.044813774725142826
exact = "πR² + π(R − a)² + 2πR(H − b) + 2π∫₀^{π/2}(R − a + a cos θ)√(a² sin²θ + b² cos²θ)dθ = 0.0111π + 0.009942096270296121"
tex = '0.0111\pi + 2\pi\int_0^{\pi/2}(0.04 + 0.01\cos\theta)\sqrt{0.01^2\sin^2\theta + 0.03^2\cos^2\theta}\,d\theta = 0.0111\pi + 0.009942096270296121'
+++

The cylinder of `fillet/elliptic-section-cylinder-rim` at a hundredth of its size: a pin a tenth of
a millimetre across, if a unit is a millimetre.

A cylinder of radius 0.05 on the z axis, from z = 0 to z = 0.1. Round the circle where its side
meets its top unequally: 0.01 in across the top and 0.03 down the side, with a section that is a
quarter ellipse.

In each half plane through the z axis the round's section is the quarter of the ellipse centered at
distance 0.04 from the axis and height 0.07, with semi-axis 0.01 across (along the radius) and 0.03
up, that runs from the top at distance 0.04 from the axis, where it is tangent to the top, to the
side at height 0.07, where it is tangent to the side: the points
(ρ, z) = (0.04 + 0.01 cos θ, 0.07 + 0.03 sin θ), 0 ≤ θ ≤ 90°, where ρ is the distance from the
axis. The round is that quarter ellipse turned round the z axis, and the corner outside it is
removed. The top is left the disk of radius 0.04 and the side ends at z = 0.07.

(An asymmetric round: the quarter ellipse is what a round of 0.01 on one face and 0.03 on the other
makes square to a straight edge between faces at 90°. Here the edge is a circle.)

**As a set:**

$$\{\, \rho \le 0.05,\ 0 \le z \le 0.1 \,\} \setminus \bigl\{\, \rho > 0.04,\ z > 0.07,\ \bigl(\tfrac{\rho - 0.04}{0.01}\bigr)^2 + \bigl(\tfrac{z - 0.07}{0.03}\bigr)^2 > 1 \,\bigr\}$$
