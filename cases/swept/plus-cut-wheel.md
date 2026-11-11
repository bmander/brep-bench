+++
title = "A wheel cut by a plus always in mesh with it"
tags = ["swept", "solid-sweep", "generating", "conjugate", "envelope", "boolean", "cut"]
origin = "geomsolver rust/examples/solid_sweeps/conjugate.sv"

[volume]
value = 17485.5244499
exact = "no closed form: cad-kernel and cadrs agree to 1e-11; the mean of their values (next.BREP is 7e-10 from it)"

[area]
value = 8193.42160418
exact = "no closed form: cad-kernel and cadrs agree to 2.5e-10; the mean of their values (next.BREP is 1.0e-8 from it)"
+++

A wheel cut by a plus-shaped tool that turns in mesh with it, as a gear shaper cuts.

The tool is a plus in the plane z = 0, centered on the origin: four arms along +x, +y, −x and −y,
each 30 long from the center and 8 wide, joined in a square 8 by 8 at the center. It is extruded from
z = −10 to z = 0.

The wheel's blank is a disk of radius 40 about the vertical line through (−50, 0), from z = −8 to
z = −2.

The tool turns about the z axis while the wheel turns two thirds as fast the other way about its own
axis: when the tool has turned right-handed by t (from +x toward +y), the wheel has turned by −2t/3
about the vertical line through (−50, 0). Over t from 0° to 540° the wheel makes one whole turn and the
tool one and a half, after which each stands as it began. Cut from the wheel everything the tool
passes through, as seen from the wheel: what is left is the material that at every instant of the
motion lies outside the tool.

The tool is never clear of the wheel.
