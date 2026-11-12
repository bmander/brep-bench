+++
title = "A 2.5-axis program of three tools: a pocket round an island cleared in raster passes, a bore and four drilled holes"
tags = ["cnc", "toolpath", "swept", "solid-sweep", "slide", "turn", "pieces", "cylinder", "cone", "coincident", "tangent", "boolean", "cut"]

[volume]
value = 257360.11215824084
exact = "160·110·20 − 12·(8400 − (4 − π)·12² − π·15²) the pocket round the island − π·9²·6 the bore − 4·(π·4²·10 + π·4²·4/3) the holes"
tex = '352000 - 12\bigl(7824 - 81\pi\bigr) - 486\pi - 4\Bigl(160\pi + \tfrac{64\pi}{3}\Bigr) = 258112 - \tfrac{718}{3}\pi'

[area]
value = 52871.636273475
exact = "2(160·110 + 160·20 + 110·20) + 12·(284 + 24π) the pocket's outer wall + 12·30π the island's wall + 2π·9·6 the bore's wall + 4·(2π·4·10 − π·4² + π·4·4√2) the holes (each floor makes up the top it opens)"
tex = '46000 + 12\,(284 + 54\pi) + 108\pi + 4\bigl(64\pi + 16\sqrt2\,\pi\bigr) = 49408 + (1012 + 64\sqrt2)\,\pi'
+++

A plate machined by a 2.5-axis program of three tools: a pocket round a round island, cleared at
three depths by raster passes and finished round its walls at each; a bore in the island's top; and
four holes drilled in the plate's corners. The third of the machining cases (`cnc/`), its program
nearly 400 moves, most of them in the air between cuts, as a real program's are.

The plate is the box from (0, 0, −20) to (160, 110, 0).

Every cutter here stays upright (along z) and only slides, never tilting. Its *tip* is the lowest
point of its axis. A move is straight unless it is an arc (G2 clockwise or G3 anticlockwise, seen
from above, about an upright line). Every move between cuts is made with the tip at z = 5, above
the plate.

**The pocket.** Its outline, $P$, is the rectangle from (20, 20) to (140, 90) with its corners
rounded to radius 12, and its island, $I$, the disc of radius 15 about (60, 55); it is 12 deep.
The cutter is a **flat end mill** of radius 5, a solid cylinder from its tip, the middle of its flat
end, 30 up. Its tip keeps to the region $D$ of points 5 or more inside $P$ and 5 or more from $I$:
inside the *wall path*, the rectangle from (25, 25) to (135, 85) with its corners rounded to radius
7, and outside the *island path*, the circle of radius 20 about (60, 55). At each depth z = −4, then
−8, then −12, the program runs, in order:

1. **Raster passes**, one along each line y = 26 + 4k, for k = 0 to 14 (y = 26, 30, …, 82), in that
   order. The line crosses $D$ in one interval, or in two where it crosses the island path
   (35 < y < 75); for each interval, from left to right, the tip moves at z = 5 to above its
   left end, plunges to the depth, runs along the line to its right end, and rises to z = 5. So
   each interval's ends lie on the wall path or the island path: at y = 26, for example, the pass
   runs from x = 32 − √13 to 128 + √13, and at y = 54 from 25 to 60 − √399 and from 60 + √399 to
   135.
2. **The wall**: the tip moves to above (80, 25), plunges to the depth, runs once round the wall
   path anticlockwise (its four sides and four arcs, the first side in two halves, from its middle
   and back to it), and rises.
3. **The island**: the tip moves to above (80, 55), plunges to the depth, runs once round the
   island path clockwise (G2, a whole circle), and rises.

**The bore.** A **flat end mill** of radius 3, 30 long from its tip, bores the island's top: the
tip moves to above (60, 55), plunges to (60, 55, −3), moves out to (66, 55, −3), runs a whole circle
of radius 6 about (60, 55) anticlockwise, moves back to (60, 55, −3), plunges to (60, 55, −6), and
does the same at z = −6: out to (66, 55, −6), a whole circle, back to (60, 55, −6); then rises.

**The holes.** A **drill** of radius 4, 30 long from its tip, a cone of half-angle 45° (a 90°
point) from its tip up to where it meets its cylindrical side, 4 above the tip, drills a hole at each of (8, 8), (152, 8), (152, 102) and (8, 102), in that order, pecking: from above the
hole it plunges to z = −5, rises to z = 2, plunges to z = −10, rises to z = 2, plunges to z = −14,
and rises to z = 5.

The solid is the plate less everything the three cutters pass through.

At each depth the passes clear $P$ less $I$ exactly, and nothing outside it: every raster pass
ends on a path the mill runs round, and every point of $D$ lies within 3, straight up or down, of
a pass or of one of those paths, less than the mill's radius; and the wall path's corners and the
island are round enough for the mill to leave nothing of $P \setminus I$ standing. So the pocket is $P \setminus I$, 12 deep, with upright walls and a
flat floor, its walls cut again and again at each depth, its floor and walls swept by many passes
in coincident and tangent faces. The bore is a hole of radius 9, 6 deep, with a flat floor; each
drilled hole is a cylinder of radius 4, 10 deep, ending in a cone 4 deep.

**As a set:** with $c_i$ the holes' axes and $\rho(h) = \min(h, 4)$ the drill's radius $h$ above
its tip,

$$\bigl([0,160]\times[0,110]\times[-20,0]\bigr) \setminus \Bigl( (P \setminus I) \times [-12, 0]
\ \cup\ \{ (x - 60)^2 + (y - 55)^2 \le 9^2,\ z \ge -6 \}
\ \cup\ \bigcup_{i=1}^{4} \{ d((x,y), c_i) \le \rho(z + 14) \} \Bigr)$$
