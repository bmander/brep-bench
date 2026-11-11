+++
title = "A ring pushed straight down into a block at a lean"
tags = ["swept", "solid-sweep", "slide", "torus", "leaning", "stopped", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/leaning_ring.sv"

[volume]
value = 1880.891253785
exact = "no closed form: cad-kernel and cadrs agree to 5e-11; the mean of their values"

[area]
value = 1186.342576165
exact = "no closed form: cad-kernel and cadrs agree to 6e-11; the mean of their values"
+++

A block, the box from (−8, −8, −8) to (8, 8, 0), with a hole bored into its top by a ring pushed
straight down into it at a lean, stopping wholly inside it.

The ring is a torus centered at (0, 0, 4), its axis leaning 35° from upright toward +x, along the
direction (sin 35°, 0, cos 35°): the circle through its tube's centers has radius 3, and its tube
has radius 1. As placed it is wholly above the block.

It slides without turning straight down by 8, so that its center ends at (0, 0, −4), the whole ring
under the block's top. Cut from the block everything the ring passes through.

The ring touches what it sweeps along two closed curves, the outer and the inner, where its normal
is level. Each is carried down into an upright wall: the hole's, and the wall of the core the ring's
own hole leaves standing in the middle.
