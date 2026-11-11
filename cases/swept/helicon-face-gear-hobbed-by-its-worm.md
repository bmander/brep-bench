+++
title = "A helicon face gear hobbed by its worm"
tags = ["swept", "solid-sweep", "generating", "gear", "worm", "face-gear", "skew-axes", "cut"]
settled = false
+++

A face gear's 30 teeth cut whole by a cylindrical worm turning in mesh with it, as a hob cuts: a
helicon pair, the worm over the gear's face on a shaft square to the gear's and 30 from it.

This is the fourth of five cases that cut a gear with its mate as the shafts move apart, from the
[straight bevel](straight-bevel-gear-cut-by-its-pinion.md) and the
[hypoid](hypoid-gear-cut-by-its-pinion.md) through the
[spiroid](spiroid-face-gear-hobbed-by-its-worm.md) to the
[worm wheel](worm-wheel-hobbed-by-its-worm.md). Its worm is the worm wheel's; here it lies over the
gear's face, where the worm wheel's lies beside its rim.

**The blank** is the ring about the z axis between radii 40 and 52, from z = −6 to z = 2: the
gear's pitch plane is z = 0, and its teeth will stand 2 above it.

**The worm's axis** is the line through (0, 30, 10) along the x axis: square to the z axis and 30
from it, 10 above the pitch plane. For a point p write x for its x coordinate, r for its distance
from the worm's axis, and θ for its angle about the axis, right-handed about +x:
(y − 30, z − 10) = r·(cos θ, sin θ).

**The tool** is the worm's thread: the points with 0 ≤ x ≤ 52, 7.5 ≤ r ≤ 12.5 and
|ξ| ≤ π/2 − (r − 10)·tan 20°, where ξ is x − θ reduced to the interval [−π, π) by a multiple of 2π.
It is a right-hand thread of lead 2π, one start: its section by any plane through the axis is a row
of trapezoids 2π apart (a rack of module 2), each π wide at radius 10 (its pitch line touches the
pitch plane along y = 30), from radius 7.5 to 12.5, its sides 20° from square to the axis. Its ends
are the planes x = 0 and x = 52, and it lies on one side of the gear's axis only, so it crosses the
ring once. The worm's core (r ≤ 7.5) never reaches the blank, so a worm with its core cuts the
same.

**The motion.** The worm turns by t about its axis, right-handed about +x (from +y toward +z),
while the gear turns by t/30 about the z axis. Seen from the gear, the worm at t is turned by t
about its axis and then by −t/30 about the z axis. t runs from 0 to 60π: thirty turns of the worm
and one of the gear, after which each stands as it began. Cut from the blank everything the worm
passes through, seen from the gear. Its ends never reach the blank.

As for the worm wheel, the thread is a screw's, so seen from the gear the worm is a rack slid along
its axis while the gear turns. But the rack now crosses the gear's face, which moves along the
worm's axis as fast as the rack only on the line y = 30, and moves across it everywhere but at
x = 0. The gear's teeth run across the face, curved, thinner at the ring's inner edge and fuller
at its outer.

**As a set:**

$$K \setminus \bigcup_{t \in [0,\,60\pi]} R_z(-\tfrac{t}{30})\,R_W(t)\,T$$

where K is the blank, T the thread and W the worm's axis.
