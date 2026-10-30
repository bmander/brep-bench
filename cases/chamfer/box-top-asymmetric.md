+++
title = "A box with its top edges chamfered unequally"
tags = ["chamfer", "asymmetric", "corner"]

[volume]
value = 1142.6666666666667
exact = "20·10·4 + ∫₀²(20 − s)(10 − s)ds = 800 + 1028/3 = 3428/3"

[area]
value = 709.2198067399883
exact = "200 + 2(20 + 10)·4 + 18·8 + 2·19√5 + 2·9√5 = 584 + 56√5"
+++

A box from the origin to (20, 10, 6). Chamfer the four edges around its top face z = 6, each with
distance 1 measured across the top face and distance 2 measured down the side face: the chamfer
of the edge at y = 10, z = 6 is the plane through the lines y = 9, z = 6 and y = 10, z = 4, and
the other three likewise.

The solid is the box cut by those four planes. Its top face is the rectangle from (1, 1, 6) to
(19, 9, 6); its four vertical edges end at z = 4, and the chamfers of two neighbouring edges meet
along a slanting line from the box's corner at z = 4 to the top face's corner, from (20, 10, 4)
to (19, 9, 6) and so on. Each chamfer face is a trapezoid; four faces meet at each of the points
such as (20, 10, 4).
