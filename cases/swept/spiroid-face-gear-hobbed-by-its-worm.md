+++
title = "A spiroid face gear hobbed by its tapered worm"
tags = ["swept", "solid-sweep", "generating", "gear", "worm", "face-gear", "skew-axes", "cut"]
settled = false
+++

A face gear's 30 teeth cut whole by a tapered worm turning in mesh with it, as a hob cuts: a
spiroid pair, the worm a single-start thread on a cone, over the gear's face on a shaft square to
the gear's and 30 from it.

This is the third of five cases that cut a gear with its mate as the shafts move apart, from the
[straight bevel](straight-bevel-gear-cut-by-its-pinion.md) and the
[hypoid](hypoid-gear-cut-by-its-pinion.md) to the
[helicon](helicon-face-gear-hobbed-by-its-worm.md) and the
[worm wheel](worm-wheel-hobbed-by-its-worm.md). Between the hypoid and the helicon, its pinion has
become a worm and has not yet lost its taper.

**The blank** is the solid of revolution, about the z axis, of the quadrilateral in the xz-plane
(ρ = x, z) with corners (40, −9), (52, −9), (52, −1.7) and (40, −0.3): a ring between radii 40 and
52 whose face is a cone falling 1.4 over its 12 of width, from z = −0.3 at its inner edge to −1.7
at its outer.

**The worm's axis** is the line through (0, 30, 6) along the x axis: square to the z axis and 30
from it. For a point p write x for its x coordinate, r for its distance from the worm's axis, and
θ for its angle about the axis, right-handed about +x: (y − 30, z − 6) = r·(cos θ, sin θ). The
worm's pitch cone has radius R(x) = 6 + x·tan 5° at x: it widens at 5° toward +x.

**The tool** is the worm's thread: the points with 0 ≤ x ≤ 52, |r − R(x)| ≤ 2.5 and
|ξ| ≤ π/2 − (r − R(x))·tan 20°, where ξ is x − θ reduced to the interval [−π, π) by a multiple
of 2π. It is a right-hand thread of lead 2π, one start, on a cone: its section by any plane through
the axis is a row of trapezoids 2π apart (a rack of module 2), each π wide at the pitch cone, from
2.5 inside it to 2.5 outside, its sides 20° from square to the axis. Its tips and roots are cones,
its ends the planes x = 0 and x = 52. It lies on one side of the gear's axis only, so it crosses the
ring once, its narrow end nearest the gear's axis. The worm's core (r ≤ R(x) − 2.5) never reaches
the blank, so a worm with its core cuts the same.

**The motion.** The worm turns by t about its axis, right-handed about +x (from +y toward +z),
while the gear turns by t/30 about the z axis. Seen from the gear, the worm at t is turned by t
about its axis and then by −t/30 about the z axis. t runs from 0 to 60π: thirty turns of the worm
and one of the gear, after which each stands as it began. Cut from the blank everything the worm
passes through, seen from the gear. Its ends never reach the blank.

Unlike the helicon's, this thread is no screw's: turned about its axis it does not slide into
itself, because its radius changes along it. Seen from the gear the worm truly turns, about an axis
square to the gear's and skew to it, so its flanks sweep envelopes of no planar motion. The
gear's teeth run across its conical face, curved.

**As a set:**

$$K \setminus \bigcup_{t \in [0,\,60\pi]} R_z(-\tfrac{t}{30})\,R_W(t)\,T$$

where K is the blank, T the thread and W the worm's axis.
