+++
title = "A flat-end mill leaning across a turn, its rims digging past its ends"
tags = ["swept", "solid-sweep", "turn", "cylinder", "sharp-edge", "leaning", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/leaning_mill.sv"

[volume]
value = 798.14204226
exact = "no closed form: cad-kernel and cadrs give the same volume to the twelve figures the board publishes"

[area]
value = 806.412078241
exact = "no closed form: cad-kernel and cadrs give the same area to the twelve figures the board publishes"
+++

A ring with a channel cut inside it by a drum leaning across a turn.

The ring is everything with distance from the z axis between 3 and 8 and with z between −2.5 and
2.5, all the way round.

The drum is a solid cylinder of radius 1 and length 3, its ends flat and square to its axis. Its
axis leans 25° from upright in the plane y = 1.2, along the direction (sin 25°, 0, cos 25°), and its
middle is the point (5, 1.2, 0).

It turns rigidly about the z axis, right-handed (from +x toward +y), by 100°, wholly inside the
ring. Cut from the ring everything the drum passes through.

The drum's side touches along two curves, each turned into a surface of revolution. Its rims are
sharp circles leaning to the turn's axis; each sweeps a ring whose section is no circle, and they
carry the cut past the flat ends' own contact lines. The channel is sealed inside the ring.
