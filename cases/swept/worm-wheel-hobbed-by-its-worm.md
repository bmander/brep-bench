+++
title = "A worm wheel hobbed by its worm"
tags = ["swept", "solid-sweep", "generating", "gear", "worm", "skew-axes", "screw", "cut"]
settled = false
+++

A worm wheel's 30 teeth cut whole by its worm turning in mesh with it, as a hob cuts: a
single-start worm of module 2 and pitch radius 10, the shafts square and 40 apart.

This is the last of five cases that cut a gear with its mate as the shafts move apart, from the
[straight bevel](straight-bevel-gear-cut-by-its-pinion.md) and the
[hypoid](hypoid-gear-cut-by-its-pinion.md) through the
[spiroid](spiroid-face-gear-hobbed-by-its-worm.md) and the
[helicon](helicon-face-gear-hobbed-by-its-worm.md). It uses the helicon's worm, and runs it beside
the wheel's rim rather than over its face.

**The blank** is the disk of radius 32 about the z axis from z = −6 to z = 6.

**The worm's axis** is the line through (0, 40, 0) along the x axis. For a point p write x for its
x coordinate, r for its distance from the worm's axis, and θ for its angle about the axis,
right-handed about +x: (y − 40, z) = r·(cos θ, sin θ).

**The tool** is the worm's thread: the points with −20 ≤ x ≤ 20, 7.5 ≤ r ≤ 12.5 and
|ξ| ≤ π/2 − (r − 10)·tan 20°, where ξ is x − θ reduced to the interval [−π, π) by a multiple of 2π.
It is a right-hand thread of lead 2π, one start: its section by any plane through the axis is a row
of trapezoids 2π apart (a rack of module 2), each π wide at radius 10, from radius 7.5 to 12.5, its
sides 20° from square to the axis. Its ends are the planes x = ±20. The worm's core (r ≤ 7.5)
never reaches the blank, so a worm with its core cuts the same.

**The motion.** The worm turns by t about its axis, right-handed about +x (from +y toward +z),
while the wheel turns by t/30 about the z axis. Seen from the wheel, the worm at t is turned by t
about its axis and then by −t/30 about the z axis. t runs from 0 to 60π: thirty turns of the worm
and one of the wheel, after which each stands as it began. Cut from the blank everything the worm
passes through, seen from the wheel. Its ends never reach the blank.

The thread is a screw's: turning it by α about its axis moves it by −α along it. So, seen from the
wheel, the worm is a rack slid along its axis while the wheel turns, and the pitch line r = 10,
z = 0 rolls on the wheel's pitch circle of radius 30: a planar motion, in the planes square to the
z axis, of a tool that is no prism. In the mid-plane z = 0 the worm's section is a straight rack, so
there the wheel's teeth are involutes of the circle of radius 30·cos 20°. Off it they lean with the
thread, and the cut is deepest in the mid-plane, where the worm comes nearest.

**As a set:**

$$K \setminus \bigcup_{t \in [0,\,60\pi]} R_z(-\tfrac{t}{30})\,R_W(t)\,T$$

where K is the blank, T the thread and W the worm's axis.
