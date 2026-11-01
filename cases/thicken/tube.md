+++
title = "A cylindrical sheet thickened into a tube"
tags = ["thicken", "sheet", "analytic", "cylinder", "seam"]

[volume]
value = 345.57519189487726
exact = "πh((R + t)² − R²), R = 5, t = 1, h = 10"
tex = '\pi h\left((R + t)^2 - R^2\right),\quad R = 5,\ t = 1,\ h = 10'

[area]
value = 760.26542216873
exact = "2πh(2R + t) + 2π((R + t)² − R²)"
tex = '2\pi h\,(2R + t) + 2\pi\left((R + t)^2 - R^2\right)'
+++

A sheet: the whole cylinder of radius 5 about the z axis from z = 0 to z = 10, one face that
closes around on itself, bounded only by its two circles, at z = 0 and z = 10. A kernel may store
the face with a seam where its parameter wraps around; the seam is no edge of the sheet, and
leaves no wall. Thicken it by 1 away from the axis.

The thickened sheet is the tube between radii 5 and 6 from z = 0 to z = 10: the sheet, the
cylinder of radius 6, and a flat annulus at each end.
