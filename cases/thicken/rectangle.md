+++
title = "A rectangular sheet thickened by 2"
tags = ["thicken", "sheet", "planar"]

[volume]
value = 160
exact = "abt, a = 10, b = 8, t = 2"
tex = 'abt,\quad a = 10,\ b = 8,\ t = 2'

[area]
value = 232
exact = "2ab + 2(a + b)t"
tex = '2ab + 2(a + b)\,t'
+++

A sheet: the rectangle in the xy plane from (0, 0, 0) to (10, 8, 0), a single flat face with no
solid behind it. Thicken it by 2 to its +z side.

The thickened sheet is the box from the origin to (10, 8, 2): the sheet, the sheet moved 2 along
+z, and the four walls its edges sweep as it moves.
