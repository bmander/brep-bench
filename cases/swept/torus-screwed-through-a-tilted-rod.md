+++
title = "A tilted rod grooved by a wheel screwed along it"
tags = ["swept", "solid-sweep", "screw", "helix", "torus", "envelope", "boolean", "cut", "adversarial"]
origin = "adversarial: swept/torus-screwed-through-rod turned rigidly so that its axis is (1, 2, 2)/3"

[volume]
value = 1460.630002527
exact = "no closed form: swept/torus-screwed-through-rod's answer (the mean of cad-kernel's, next.BREP's and cadrs's values, which agree to 7e-10), the solid being that one turned rigidly"

[area]
value = 791.5632225563
exact = "no closed form: swept/torus-screwed-through-rod's answer (the mean of cad-kernel's, next.BREP's and cadrs's values, which agree to 1e-9), the solid being that one turned rigidly"
+++

The rod and groove of `swept/torus-screwed-through-rod`, turned rigidly about the origin so that the
rod's axis is no longer a coordinate axis. Write u = (2, −2, 1)/3, v = (2, 1, −2)/3 and w = (1, 2, 2)/3: three unit vectors square to one another, right-handed (u × v = w). The point that case puts at (x, y, z) is here at
x·u + y·v + z·w.

So: a rod, the cylinder of radius 5 about the line through the origin along w, from the origin to
20w = (20/3, 40/3, 40/3), with a helical groove ground around it by a wheel carried along a screw,
as a thread is ground.

The wheel is a torus: the circle through its tube's centers has radius 6, and its tube has radius 2.
As placed, its center is 11.5u = (23/3, −23/3, 23/6), so the part of its tube nearest the rod is
centered at 5.5u = (11/3, −11/3, 11/6) and reaches 1.5 into the rod. Its axis passes through its
center square to u, along cos h·v − sin h·w, where h = atan(2π·5.5 / 30) ≈ 49.04° is the helix
angle. The wheel then lies along the helix that the screw below carries the point 5.5u along, as a
grinding wheel is set.

The screw is about the rod's axis: the wheel turns right-handed about w (from u toward v) and rises
30 along w for every whole turn, rigidly. It runs from a turn of −108° (the wheel lowered 9, wholly
below the rod) to a turn of +348° (raised 29, wholly above it). Cut from the rod everything the
wheel passes through.

The groove's section square to the axis is the same at every station along it, turned.
