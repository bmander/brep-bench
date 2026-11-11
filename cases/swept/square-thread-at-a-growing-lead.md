+++
title = "A rod threaded at a lead that grows along it"
tags = ["swept", "solid-sweep", "general-motion", "helix", "prism", "variable-pitch", "boolean", "cut"]
origin = "geomsolver #721, box K2 (any rigid motion)"

[volume]
value = 1324.478271075
exact = "no closed form: OCCT and cadrs agree to 8e-10; the mean of their values"

[area]
value = 808.437626175
exact = "no closed form: OCCT and cadrs agree to 2e-10; the mean of their values"
+++

A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, threaded by a bar carried
along a path whose lead grows as it climbs, as a variable-pitch screw is cut.

The bar, as placed, is the box 4 ≤ x ≤ 12, −8 ≤ y ≤ 8, −1 ≤ z ≤ 1: it points at the rod's axis, its
flat tip 1 inside the rod, and it is 2 wide along the axis.

When it has turned by θ about the z axis, right-handed (from +x toward +y), it has risen 6s + 0.3s²,
s = θ/360°, rigidly. θ runs from −90° (risen −1.48125, the bar wholly below the rod) to 1260° (risen
24.675, wholly above it). Cut from the rod everything the bar passes through.

The lead grows from about 6 at the bottom to about 8 at the top: no screw carries the bar, and the
groove's section is different at every height.
