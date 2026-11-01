+++
title = "A plate with rounded corners shelled as thick as its rounds: the inside rounds shrink to edges"
tags = ["shell", "inward", "round", "degenerate", "vanishing-face"]

[volume]
value = 1485.6637061435918
exact = "10(264 + 4π) − 8·16·10 = 1360 + 40π"
tex = '10(264 + 4\pi) - 8\cdot 16\cdot 10 = 1360 + 40\pi'

[area]
value = 1614.79644737231
exact = "2(264 + 4π) + 10(52 + 4π) + 8·52 = 1464 + 48π"
tex = '2(264 + 4\pi) + 10(52 + 4\pi) + 8\cdot 52 = 1464 + 48\pi'
+++

The box from the origin to (20, 14, 10) with its four vertical edges rounded to radius 2: a plate
whose outline is the 20 × 14 rectangle with its corners rounded to radius 2, 10 high. Shell it
inward to a wall 2 thick, open at its top face, z = 10. The outside of the wall is the plate's other
faces, its rounds among them, unmoved, and the inside is each of them moved 2 inward along its
normal, onto the surface offset 2 from it. The moved faces are extended or trimmed where they meet
one another, and run on until they meet the surface of an opening, which is left as a rim between
the outside and the inside.

Each round, moved 2 inward, is the cylinder of radius 0 on its axis: it has shrunk to a line, and
the inside walls on either side of it meet there in a sharp edge. So the cavity is the sharp-
cornered box from (2, 2, 2) to (18, 12, 10), and the inside has no rounded face: its corners are the
rounds' axes. A kernel that keeps a round of zero radius as a face, or fails to offset it, does not
make it.
