+++
title = "A box with a half-cylinder boss on top, all edges filleted"
tags = ["feature", "boolean", "union", "fillet", "half-cylinder", "boss", "unsettled"]
settled = false
+++

A box 60 × 60 × 10 sitting on the xy plane with one corner at the origin. Its top face lies in the plane z = 10.

On the top face, along the edge that runs parallel to the y axis at x = 30, z = 10 (the middle of the front-to-back edge), place a half-cylinder boss. The boss is a half-cylinder of radius 15 whose flat face lies in the plane z = 10. Its axis runs along the y direction, centred at y = 30 (the midpoint of that edge), and its length along y is 10 mm, so it spans y = 25 to y = 35. The cross-section in the xz plane is a half-circle of radius 15 with the diameter on z = 10 and the arc extending upward to z = 25, centred at x = 30. Unite the boss with the box.

Finally, fillet every sharp edge of the resulting solid with a constant radius of 2 mm.