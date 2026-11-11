+++
title = "A hypoid gear's tooth space cut by its pinion"
tags = ["swept", "solid-sweep", "generating", "gear", "hypoid", "skew-axes", "boolean", "cut"]
settled = false
+++

The [straight bevel gear](straight-bevel-gear-cut-by-its-pinion.md)'s blank with one tooth space
cut by a pinion tooth on a shaft that misses the gear's: shafts square, 12 apart, a 30-tooth gear
and a 14-tooth pinion. It is the second of five cases that cut a gear with its mate as the shafts
move apart, from bevel to worm.

The gear's axis is the z axis. Write g = (2, 0, 1)/√5 and d = (−1, 0, 2)/√5 as in the straight
bevel case.

**The blank** is that case's blank: the solid of revolution, about the z axis, of the quadrilateral
in the xz-plane (ρ = x, z) with corners (30.894427, 13.211146), (27.316718, 20.366563),
(18.372447, 15.894427) and (21.683489, 9.272343), the points s·g + u·d at s = 15√5, u = −2;
s = 15√5, u = 6; s = 15√5 − 10, u = 6; and s = 15√5 − 10, u = −2(15√5 − 10)/(15√5).

**The pinion's axis** is the line through (0, 12, 0) along the x axis: square to the z axis, and 12
from it along their common perpendicular, the y axis. That distance is the hypoid's offset.

**The tool** is the straight bevel case's tooth turned about the z axis by φ, where
sin φ = 24/(75 − 5√5), so φ ≈ 22.0898°. That tooth is the frustum of the pyramid with apex O (the
origin) over the trapezoid of the points P + u·d + w·(0, 1, 0), P = (30, 0, 15), with |u| ≤ 2.5 and
|w| ≤ π/2 − u·tan 20° (corners (28.881966, ±0.660871, 17.236068) and
(31.118034, ±2.480721, 12.763932)), cut off by the planes square to g at distances 15√5 − 12 and
15√5 + 2 from O. Turned, it lies along the gear's pitch cone at azimuth φ, its flanks still planes
through O.

The angle places the tooth where the pinion can carry it: at the tooth's mean point M, halfway
across the face (cone distance 15√5 − 5), the line through M square to the gear's pitch cone meets
the pinion's axis. So the pitch surfaces touch at M, and there the tooth neither rises nor sinks
as it passes. Its length runs along the gear's cone, through O, and so leans to the pinion's: the
pinion's teeth are spiral where the gear's are straight.

**The motion.** The pinion turns by t about its axis, right-handed about +x (from +y toward +z),
while the gear turns by −7t/15 about the z axis. Seen from the blank, the tooth at t is turned by
t about the pinion's axis and then by 7t/15 about the z axis. As t runs from −70° to 70° the tooth
comes into the blank, passes through the mesh and leaves it; at both ends it is clear of the blank.
Cut from the blank everything the tooth passes through, seen from the blank.

The tooth slides along its length at M, as every hypoid tooth does. It would slide nowhere across
itself there at the ratio cos φ / 2 ≈ 0.4633; the case runs at the tooth counts' 7/15 ≈ 0.4667, as
a pair is made, so it slides a little across as well. The turns no longer hold a point in common:
the space is no cone, and its flanks are envelopes of planes under a motion whose axes are skew.

**As a set:**

$$K \setminus \bigcup_{t \in [-70^\circ,\,70^\circ]} R_z(\tfrac{7t}{15})\,R_A(t)\,R_z(\varphi)\,T$$

where K is the blank, T the straight bevel case's tooth and A the pinion's axis.
