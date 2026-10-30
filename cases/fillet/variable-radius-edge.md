+++
title = "A cube edge rounded with a radius growing along it"
tags = ["fillet", "variable-radius"]

[volume]
value = 994.9926238126071
exact = "1000 − (1 − π/4)∫₀¹⁰(1 + z/10)²dz = 1000 − (70/3)(1 − π/4)"
tex = '1000 - \left(1 - \tfrac{\pi}{4}\right)\int_0^{10}\left(1 + \tfrac{z}{10}\right)^2 dz = 1000 - \tfrac{70}{3}\left(1 - \tfrac{\pi}{4}\right)'

[area]
value = 592.4995517137655
exact = "600 − 2∫₀¹⁰ r dz − (1 − π/4)(1² + 2²) + ∫₀¹⁰ r dz·∫₀^{π/2}√(1 + (r′)²(1 − sin θ − cos θ)²)dθ, r = 1 + z/10, r′ = 1/10: 570 − 5(1 − π/4) + 15·I, I = ∫₀^{π/2}√(1 + (1 − sin θ − cos θ)²/100)dθ = 1.5715040597852221 (the round's area separates as ∫ r dz times I)"
tex = '\begin{array}{l} 570 - 5\left(1 - \tfrac{\pi}{4}\right) + 15I \\ I = \displaystyle\int_0^{\pi/2}\sqrt{1 + \tfrac{(1 - \sin\theta - \cos\theta)^2}{100}}\,d\theta = 1.5715040597852221 \end{array}'
+++

A cube of side 10 between the origin and (10, 10, 10). Round the edge where its faces x = 10 and
y = 10 meet, the line from (10, 10, 0) to (10, 10, 10), with a radius that grows linearly along
it: r = 1 + z/10, so 1 at the bottom face z = 0 and 2 at the top face z = 10.

The round's sections square to the edge are circular: at each height z the round's section by
the horizontal plane is the quarter circle of radius r(z) centred at (10 − r, 10 − r, z), tangent
to the face x = 10 at (10, 10 − r, z) and to the face y = 10 at (10 − r, 10, z). (So the round is
part of an oblique circular cone with its apex at (10, 10, −10), tangent to both faces along
straight lines.) It cuts the bottom face with the quarter circle of radius 1 and the top face
with the quarter circle of radius 2.
