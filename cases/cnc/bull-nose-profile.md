+++
title = "A groove profiled round a six-lobed flower by a bull-nose mill in three step-downs"
tags = ["cnc", "toolpath", "swept", "solid-sweep", "turn", "pieces", "torus", "coincident", "boolean", "cut"]

[volume]
value = 208489.64862880707
exact = "120·120·16 − A·L, A = 2·4·10 − (4 − π)·1.5²/2 the groove's section (6 wide less two corners rounded to 1.5), L = 6(12(2π − 2α) + 2(d − 12)β) the tip's path, d = √(2500 − 1200√3), cos α = (30² + d² − 40²)/(60d), cos β = (40² + d² − 30²)/(80d) (Pappus: the section is the same all round, symmetric about the path)"
tex = '120^2\cdot 16 - \Bigl(80 - \tfrac{(4 - \pi)\,1.5^2}{2}\Bigr) L,\quad L = 6\bigl(12(2\pi - 2\alpha) + 2(d - 12)\beta\bigr),\quad d = \sqrt{2500 - 1200\sqrt3},\ \cos\alpha = \tfrac{30^2 + d^2 - 40^2}{60d},\ \cos\beta = \tfrac{40^2 + d^2 - 30^2}{80d}'

[area]
value = 41667.55857176368
exact = "2(120·120 + 2·120·16) + L(2·(10 − 1.5) + π·1.5 + 2·(4 − 1.5) − 2·4): the walls, the corner rounds and the floor along the path, less the top the groove opens"
tex = '36480 + L\,(14 + 1.5\pi)'
+++

A plate with a groove milled round a six-lobed flower by a bull-nose mill, at three depths. The
second of the machining cases (`cnc/`): the path is all arcs, convex and concave, and the mill's
rounded corner sweeps tori round them.

The plate is the box from (0, 0, −16) to (120, 120, 0).

The cutter is a **bull-nose mill** of radius 4 whose end corner is rounded to radius 1.5: about its
axis, which stays upright throughout, its *tip* the middle of its flat end, its flat end a disc of
radius 2.5, joined to its side (radius 4, from 1.5 above the tip up to 25 above it, where it is flat)
by a quarter of a torus whose tube, of radius 1.5, is centered on the circle of radius 2.5 1.5
above the tip.

**The flower** is a closed path of arcs about the upright line through (60, 60), in the plane of
the tip:

- six *lobes*, each an arc of radius 12 about a point 30 from (60, 60), at 0°, 60°, 120°, 180°,
  240° and 300° from +x toward +y: the lobe at 0° is about (90, 60);
- six *valleys*, each an arc of radius d − 12 about a point 40 from (60, 60), at 30°, 90°, 150°,
  210°, 270° and 330°, where d = √(2500 − 1200√3) ≈ 20.531 is the distance from a lobe's center to
  a neighbouring valley's, so that each valley is tangent to the two lobes beside it: its radius is
  about 8.531.

Each lobe runs between its tangent points with its two valleys, on the side of its center away from
(60, 60), and each valley between its tangent points with its two lobes, on the side of its center
toward (60, 60): the flower keeps out of each valley's circle, and is concave there. The flower is
smooth all round, each lobe a convex arc of about 154°, each valley a concave arc of about 94°.

The program moves the tip through these positions in order, the mill only sliding (never tilting):

1. The tip starts at (102, 60, 5), above the outermost point of the lobe at 0°, and plunges straight
   down to (102, 60, −4).
2. **A lap round the flower** at its own depth, anticlockwise seen from above: along the lobe at 0°
   anticlockwise (G3) to its tangent point with the valley at 30°, round that valley clockwise (G2)
   to the lobe at 60°, and so round, each lobe anticlockwise and each valley clockwise, back along
   the lobe at 0° to (102, 60). Each lap is thirteen arcs, the lobe at 0° run in two pieces.
3. The tip plunges to (102, 60, −8) and runs a second lap at z = −8; then plunges to
   (102, 60, −10) and runs a third at z = −10.
4. It rises straight up to (102, 60, 5).

The solid is the plate less everything the mill passes through.

Every arc of the flower has a radius more than the mill's 4 (12 and about 8.531), and the flower's
parts are far apart, so the groove has the same section all round: 8 wide and 10 deep, its floor
5 wide at z = −10, its two bottom edges rounded to 1.5 by the mill's corner, its walls upright. The
two shallower laps' grooves lie wholly within the deepest's, their walls on its walls.

**As a set:** with $F$ the flower, the tip's path in the plane z = 0, and the mill's radius at a
height $h$ above its tip $\rho(h) = 2.5 + \sqrt{1.5^2 - (1.5 - h)^2}$ for $0 \le h \le 1.5$, 4
above,

$$\bigl([0,120]^2\times[-16,0]\bigr) \setminus \bigl\{\, z \ge -10,\ d((x,y), F) \le \rho(z + 10) \,\bigr\}$$
