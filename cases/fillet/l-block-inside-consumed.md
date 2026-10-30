+++
title = "An L-block whose inside corner is rounded away"
tags = ["fillet", "concave", "vanishing-face"]

[volume]
value = 615.1548999352503
exact = "10·(10² − πr²/4), r = 7: the section is the 10 × 10 square less a quarter disc of radius 7"

[area]
value = 492.98672286269283
exact = "2·(100 − 49π/4) + 10·(10 + 3 + 7π/2 + 3 + 10) = 460 + 21π/2"
+++

An L-shaped block. Its section in the xz plane is the polygon through (0, 0, 0), (10, 0, 0),
(10, 0, 3), (3, 0, 3), (3, 0, 10) and (0, 0, 10); extrude it along +y from y = 0 to y = 10. The
block has one concave edge: the inside corner, the line x = 3, z = 3 along y, where the face
z = 3 (from x = 3 to x = 10) meets the face x = 3 (from z = 3 to z = 10).

Round that inside edge to radius 7, adding material. The ball rolling in the corner touches the
face z = 3 at x = 10 and the face x = 3 at z = 10, the far edges of both faces, so the round
consumes both faces entirely: the round is a quarter of the cylinder of radius 7 on the line
x = 10, z = 10, and it meets the faces x = 10 and z = 10 at right angles along the lines
(10, y, 3) and (3, y, 10). The section becomes the square from (0, 0) to (10, 10) less the
quarter disc of radius 7 centred at its corner (10, 10).
