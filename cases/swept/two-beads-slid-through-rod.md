+++
title = "A rod grooved by a wheel of two beads"
tags = ["swept", "solid-sweep", "slide", "torus", "boolean", "union", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/union.sv"

[volume]
value = 1486.4498667518676
exact = "no closed form taken (the rod's disc less two overlapping discs): OCCT, arris and cad-kernel, built independently from this description, agree to 8e-16 (1486.4498667518674 and 1486.4498667518671 and 1486.4498667518683; the mean)"

[area]
value = 787.2349038041657
exact = "no closed form taken (the rod's disc less two overlapping discs): OCCT, arris and cad-kernel, built independently from this description, agree to 2e-15 (787.234903804165 and 787.234903804165 and 787.234903804167; the mean)"
+++

A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, grooved along its length by a
wheel of two beads slid past it. The beads are two tori on one axis, which runs parallel to the
y axis through (11.5, 0, −9):

- the first bead's centre is (11.5, 0, −9); the circle through its tube's centres has radius 6, and
  its tube has radius 2;
- the second bead's centre is (11.5, 1.5, −9), 1.5 further along the axis; the circle through its
  tube's centres also has radius 6, and its tube has radius 1.5.

The two beads overlap, and the wheel is their union. It starts below the rod, as placed, and slides
without turning 40 along the z axis, up to where it is wholly above the rod. Cut from the rod
everything the wheel passes through.
