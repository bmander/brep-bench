+++
title = "A regular tetrahedron carried along a screw"
tags = ["swept", "solid-sweep", "screw", "helix", "polyhedron", "vertex"]
origin = "geomsolver #721, box G1 (vertices)"

[volume]
value = 153.4631140397
exact = "no closed form: cad-kernel, OCCT and cadrs agree to 2e-10; the mean of their values"

[area]
value = 324.9050838577
exact = "no closed form: cad-kernel, OCCT and cadrs agree to 7e-11; the mean of their values"
+++

The solid a regular tetrahedron fills as it is carried along a screw.

The tetrahedron's corners are (5, 1, 1), (5, −1, −1), (3, 1, −1) and (3, −1, 1): the regular
tetrahedron of edge 2√2 inscribed in the cube from (3, −1, −1) to (5, 1, 1).

The screw is about the z axis: the tetrahedron turns right-handed about it (from +x toward +y) and
rises 5 for every whole turn, rigidly, from a turn of 0° to one of 540°. Make the solid of
everything the tetrahedron passes through.

Its silhouette under the screw turns at its corners, so the swept solid's walls are sheets swept by
its edges, joined where the corners' helices run, and the turns of the screw stay apart (it rises 5
a turn and is 2 tall).
