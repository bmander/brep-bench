+++
title = "A pocketed tray with four blind-bored standoffs"
tags = ["part", "extrude", "boolean", "cut", "union", "planar", "analytic"]
origin = "geomsolver rust/examples/solid_tray.sv"

[volume]
value = 21371.150263212312
exact = "72·48·16 − 66·42·13 + 4·π(5² − 2²)·8 = 19260 + 672π"

[area]
value = 14967.43350880821
exact = "2·72·48 + 2·(72 + 48)·16 + 2·(66 + 42)·13 + 4·(2π·5·8 + 2π·2·8) = 13560 + 448π"
+++

An open tray lying on its back, its opening facing −y. It is made in three steps.

1. A block from (−36, 0, −24) to (36, 16, 24): 72 in x, 16 in y and 48 in z.
2. A pocket taken out of it: the box from (−33, 0, −21) to (33, 13, 21). This leaves walls 3
   thick all round, and a floor 3 thick from y = 13 to y = 16. The pocket opens through the
   block's face at y = 0.
3. Four standoffs joined to the tray, standing on the floor inside the pocket. Each is a tube
   along y, outer radius 5 and inner radius 2, from y = 5 to y = 13: it stands 8 tall on the
   floor's inside face (y = 13), toward the opening. Their axes are the lines parallel to y
   through (x, z) = (26, 14), (−26, 14), (−26, −14) and (26, −14). Each tube's hole is blind: it
   runs from y = 5 down to the floor and stops there, and the floor under it is left whole.
   Each standoff clears the pocket's walls by 2.
