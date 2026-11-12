+++
title = "A groove profiled round a cross by a flat end mill in three step-downs"
tags = ["cnc", "toolpath", "swept", "solid-sweep", "slide", "turn", "pieces", "cylinder", "coincident", "boolean", "cut"]

[volume]
value = 147468.74388142145
exact = "120·90·15 − 2r·d·L, r = 3, d = 9, L = 156 + 36π the tip's path round the cross (Pappus: the groove's section, 2r by d, is the same all round, symmetric about the path)"
tex = '120\cdot 90\cdot 15 - 2\cdot 3\cdot 9\,(156 + 36\pi) = 153576 - 1944\pi'

[area]
value = 32743.752039526185
exact = "2(120·90 + 120·15 + 90·15) + 2d·L: the groove's floor makes up the top it takes away, and its walls, d high, run along the path's two offsets, which together are 2L long"
tex = '27900 + 18\,(156 + 36\pi) = 30708 + 648\pi'
+++

A plate with a groove milled round a cross, as a CNC program cuts it: a flat end mill taken round
the cross's outline at three depths, each lap a little deeper than the last. The first and easiest
of the machining cases (`cnc/`), which grow to a five-axis program of hundreds of passes; each takes
the plate less **everything its cutters pass through**.

The plate is the box from (0, 0, −15) to (120, 90, 0).

The cutter is a **flat end mill** of radius 3: a solid cylinder about its axis, which stays upright
(along z) throughout, from its *tip*, the middle of its flat end, 20 up.

The program moves the tip through these positions in order. A move is straight (G1) unless it is
an arc (G2 or G3), and along any move the mill only slides: it never tilts.

1. The tip starts at (60, 10, 5), above the plate, and plunges straight down to (60, 10, −3).
2. **A lap round the cross.** The cross is the outline through (45, 10), (75, 10), (75, 30),
   (100, 30), (100, 60), (75, 60), (75, 80), (45, 80), (45, 60), (20, 60), (20, 30), (45, 30)
   and back to (45, 10): a bar 80 by 30 across a bar 30 by 70, its twelve corners eight outside
   ones and four inside ones. Every corner is rounded to radius 6, by a quarter circle tangent to
   both its sides. The tip runs once round the rounded outline at its own depth, anticlockwise
   seen from above: from (60, 10) along +x, the arc round each corner (anticlockwise round an
   outside corner, clockwise round an inside one), and so back to (60, 10). Each lap is twelve
   arcs and the twelve straight sides between them, the first side run in two halves, from its
   middle and back to it.
3. The tip plunges to (60, 10, −6) and runs a second lap at z = −6; then plunges to (60, 10, −9)
   and runs a third at z = −9.
4. It rises straight up to (60, 10, 5).

The solid is the plate less everything the mill passes through.

Every arc of the path has a radius of 6, more than the mill's 3, and its sides are far apart, so
the groove has the same section all the way round: 6 wide and 9 deep, its floor at z = −9 and its
walls square to the floor. The three laps cut the same walls: the shallower laps' walls lie wholly
on the deepest's.

**As a set:** with $C$ the rounded cross's outline, the tip's path in the plane z = 0,

$$\bigl([0,120]\times[0,90]\times[-15,0]\bigr) \setminus \bigl\{\, d((x,y), C) \le 3,\ z \ge -9 \,\bigr\}$$
