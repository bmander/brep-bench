+++
title = "A free-form surface finished by a ball-end mill in 41 three-axis raster passes"
tags = ["cnc", "toolpath", "swept", "solid-sweep", "slide", "pieces", "sphere", "cylinder", "scallop", "boolean", "cut"]
settled = false
+++

A plate whose top is finished into a free-form surface by a ball-end mill in raster passes, as a
three-axis CAM program does it: the surface sampled along each pass and the tip run straight from
point to point. The fourth of the machining cases (`cnc/`): its moves are no longer level, so what
the mill sweeps is aslant, and its 944 moves overlap their neighbours across and along the passes,
leaving the scallops a ball-end mill leaves between passes. It has no closed form.

The plate is the box from (0, 0, −20) to (100, 80, 0).

The cutter is a **ball-end mill** of radius 3: about its axis, which stays upright (along z)
throughout, a half ball of radius 3 whose lowest point is its *tip*, below a cylinder of radius 3
from 3 above the tip to 30 above it, where it is flat. It only slides, never tilting.

**The surface** the tip is run over is

$$f(x, y) = -5 + 4 \sin\frac{\pi x}{50} \sin\frac{\pi y}{40},$$

two hills and two hollows, 4 high and 4 deep about z = −5, meeting along the lines x = 50 and
y = 40.

**The passes.** There are 41 passes, the pass $j$ (from 0 to 40) along the line y = 2j. Along it
the tip runs through the 23 points $(x_i, 2j, f(x_i, 2j))$, $x_i = -5 + 5i$ for $i$ = 0 to 22, from
x = −5 to x = 105, so that it starts and ends off the plate: in turn, straight from each point to
the next. The even passes run along +x, from $i = 0$ to 22, and the odd ones back along −x, from
$i = 22$ to 0 (a zigzag).

**The program**, in order:

1. The tip starts at (−5, 0, 5) and plunges straight down to the first point of pass 0, (−5, 0, −5).
2. Pass 0, then each pass after it: from the last point of a pass, straight to the first point of
   the next, off the plate (at x = 105 or −5, 2 along y), and along that pass.
3. From the last point of pass 40, (−5, 80, −5), the tip rises straight up to (−5, 80, 5).

The solid is the plate less everything the mill passes through.

The passes are 2 apart, a third of the mill's diameter, so the ball leaves ridges (scallops) between
them, about 0.17 high on level ground and more where the surface slopes across the passes. Along a
pass, between its points, the mill runs straight while the surface curves, so where the surface is
convex along the pass it is cut by flat chords and where it is concave the ball leaves crests at
the points. Every part of the plate's top the mill reaches is cut: the surface lies below the top
everywhere, and the passes cover the plate from edge to edge.
