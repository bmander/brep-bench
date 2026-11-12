+++
title = "A dome machined in five axes by four cutters in 238 passes: roughed in levels, finished along its meridians and round its pole, spot-faced and drilled square to it"
tags = ["cnc", "toolpath", "five-axis", "swept", "solid-sweep", "slide", "turn", "pieces", "sphere", "torus", "cylinder", "cone", "scallop", "coincident", "boolean", "cut"]
settled = false
+++

A round blank machined into a dome by a five-axis program of four cutters: roughed by a bull-nose
mill in level rings, finished by a ball-end mill held square to the dome along 180 meridians and
round 15 circles near its pole, then spot-faced and drilled at twelve points, each square to the
dome. The last and hardest of the machining cases (`cnc/`): 238 passes and over a thousand moves, most
of them with the cutter leaning, the finishing passes crowding together toward the pole until
neighbours lie a third of a millimetre apart. It has no closed form.

The program is written for a machine whose table tilts and turns the part under an upright
spindle (a trunnion). Here it is given as the cutter's motion **relative to the part**, in the
part's own coordinates, where each move is one of three rigid motions at a steady rate:

- a **slide**, the cutter carried straight without turning;
- a **turn about the z axis**, the table's own axis, the cutter carried rigidly round it;
- a **turn about a level line through the origin**, the trunnion's axis, the cutter carried rigidly
  round it.

The blank is the solid cylinder of radius 50 about the z axis from z = 0 to z = 45. The dome the
program aims at is the ball of radius 40 about the origin, above z = 0.

A cutter is a solid of revolution about its axis, which it carries with it as it moves. Its *tip*
is the lowest point of its axis, and its axis runs from the tip up the cutter. A point on the dome
is named by its latitude λ above the plane z = 0 and its longitude φ from +x toward +y; the dome's
outward normal there is $n(\lambda, \varphi) = (\cos\lambda \cos\varphi, \cos\lambda \sin\varphi,
\sin\lambda)$. Wherever the program leaves a cutter at one point and takes it up at another (between
passes, seats and holes not given below), it carries it clear of the blank: at 90 or more from the
origin, or 70 for the seats and holes, the cutter reaching away from the origin, or at z = 50 with
its axis upright. The blank's farthest point is $\sqrt{50^2 + 45^2} \approx 67.3$ from the origin,
so no such move cuts anything; the moves given that way are given so that the program is whole.

**1. Roughing.** A **bull-nose mill** of radius 8 whose end corner is rounded to radius 2 (its flat
end a disc of radius 6, joined to its side, from 2 above the tip to 40 above it, by a quarter of a
torus of tube radius 2 centered on the circle of radius 6 two above the tip), its axis upright
(along z). It cuts eight levels, its tip at z = 41, 36, 31, 26, 21, 16, 11 and 6, in that order. At
the level z, where the dome's radius is $\rho = \sqrt{\max(0,\ 40^2 - z^2)}$, it runs circles of
radius $R = \rho + 9 + 12m$ for m = 0, 1, 2, … while R < 58, the largest first: for each, the tip
moves at z = 50 to above (R, 0), plunges to (R, 0, z), turns once round the z axis, anticlockwise
seen from above, and rises to (R, 0, 50). Eighteen circles in all: five at z = 41, three at z = 36,
two at each of z = 31, 26, 21 and 16, and one at each of z = 11 and 6.

**2. Finishing.** A **ball-end mill** of radius 4: a half ball of radius 4 whose lowest point is its
tip, below a cylinder of radius 4 from 4 above the tip to 40 above it, where it is flat. It is held
square to the dome, its axis along the normal: at the point (λ, φ) its tip is at $40\,n(\lambda,
\varphi)$, where its ball touches the dome, and its axis is $n(\lambda, \varphi)$.

- **Along the meridians**, 180 passes, at φ = 0°, 2°, 4°, … 358°, in that order. For each: the tip
  starts at $90\,n(3°, \varphi)$ and slides along the axis to $40\,n(3°, \varphi)$; the cutter
  turns about the level line through the origin along $(\sin\varphi, -\cos\varphi, 0)$, by 72°,
  right-handed, so that its point of contact climbs the meridian from latitude 3° to 75°; it
  slides out along its axis to $90\,n(75°, \varphi)$, turns back about the same line by 72° to
  $90\,n(3°, \varphi)$, and turns about the z axis by 2°, anticlockwise, to $90\,n(3°, \varphi +
  2°)$, where the next pass starts.
- **Round the pole**, 15 passes, at λ = 75°, 76°, … 89°, in that order. For each: the tip slides
  from $90\,n(\lambda, 0°)$ along the axis to $40\,n(\lambda, 0°)$, the cutter turns once round the
  z axis, anticlockwise, and slides back out to $90\,n(\lambda, 0°)$.
- **At the pole**, the tip slides from (0, 0, 90) down to (0, 0, 40), its axis upright, and back
  up.

**3. Spot faces.** A **flat end mill** of radius 5, a solid cylinder from its tip, the middle of its
flat end, 40 up, faces a seat at each of the twelve points at latitude 40° and longitude φ = 15°,
45°, 75°, … 345°, in that order: its axis along $n(40°, \varphi)$, its tip slides from
$70\,n(40°, \varphi)$ to $38.5\,n(40°, \varphi)$ and back.

**4. Holes.** A **drill** of radius 2.5, 40 long from its tip, a cone of half-angle 45° (a 90°
point) from its tip up to where it meets its cylindrical side, 2.5 above the tip, drills a hole at
each of the same twelve points, in the same order: its axis along $n(40°, \varphi)$, its tip slides
from $70\,n(40°, \varphi)$ to $30\,n(40°, \varphi)$ and back.

The solid is the blank less everything the four cutters pass through.

The roughing leaves the dome under steps 5 high, a little proud of it. Each meridian pass's ball
sweeps a band of a torus about its own level line, touching the dome along the meridian, and its
shank sweeps away what the roughing left beyond the ball; the passes are 2° of longitude apart, so
the ball leaves scallops between them, low near the equator and vanishing toward latitude 75°,
where they lie a third of a millimetre apart. The fifteen passes round the pole finish the cap the
meridians do not reach, and the last touch, at the pole itself, takes the stub the roughing left
there. Each seat is a flat disc of radius 5 square to the normal, 1.5 below the dome at its centre,
and each hole runs along the normal from the seat's floor to its tip, 10 below the dome, ending in a
cone.
