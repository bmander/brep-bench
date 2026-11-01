+++
title = "A box hollowed, its cavity closed"
tags = ["shell", "inward", "planar", "void", "closed"]

[volume]
value = 288
exact = "abc − (a − 2t)(b − 2t)(c − 2t), a = 10, b = 8, c = 6, t = 1"
tex = 'abc - (a - 2t)(b - 2t)(c - 2t),\quad a = 10,\ b = 8,\ c = 6,\ t = 1'

[area]
value = 584
exact = "2(ab + bc + ca) + 2(8·6 + 6·4 + 4·8)"
tex = '2(ab + bc + ca) + 2(8\cdot 6 + 6\cdot 4 + 4\cdot 8)'
+++

The box from the origin to (10, 8, 6), hollowed to a wall 1 thick with no opening: every face stays
as the outside, and each is moved 1 inward along its normal to make the inside. The cavity is
closed: the result is the box less the box from (1, 1, 1) to (9, 7, 5), one solid bounded by two
shells, the outer one facing out and the cavity's facing in, towards the void.

A volume taken by the divergence theorem counts the cavity's shell negatively, so a cavity whose
faces face the wrong way, or one left out, shows in the volume.
