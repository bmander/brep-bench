+++
title = "#3DBenchy"
tags = ["integration", "part", "reference", "loft", "boolean", "cut", "union", "common"]
tolerance = 2e-5
reference = "benchy.step.gz"
reference_mesh = "benchy.stl.gz"
origin = "#3DBenchy by Daniel Norée (2015), CC0 1.0: its original CAD file as released in STEP for its tenth anniversary, the FreeCAD export '3DBenchy by Daniel Norée - CC0 1.0 Universal' of 2025-01-31 (printables.com/model/1263390, 3DBenchyCAD.step, SHA-256 ff85d227…)"

[volume]
value = 15550.999869574227
exact = "no closed form: the reference's own, as the harness reads its STEP file (harness/mass.py, by its edges' curves, as ISO 10303-42 has them); Open CASCADE, which reads it by curves it fits on the faces, comes to 15550.9534 (3e-6 off)"

[area]
value = 9432.403647291821
exact = "the reference's own, as the harness reads it; Open CASCADE comes to 9432.3821 (2e-6 off)"

[centroid]
value = [1.3799708802954305, -0.0005574275340702653, 13.523699983833676]
exact = "the reference's own, as the harness reads it; Open CASCADE's is 1.1e-5 of the radius of gyration (18.6) off"

[inertia]
value = [[2584031.6565697175, 74.95663379742419, -620367.5396081291],
         [74.95663379742419, 4730170.961831303, -122.42253597895335],
         [-620367.5396081291, -122.42253597895335, 3412891.862734881]]
exact = "about the centroid, of unit density: the reference's own, as the harness reads it; Open CASCADE's is 5e-6 of its largest term off"
+++

The tugboat that 3D printers are tested with: #3DBenchy, from its designer's own CAD file, now in
the public domain. It is a reference shape, not a description: the solid is the one in
`benchy.step.gz` (beside this file), one closed B-rep of 1385 faces (432 planes, 75 cylinders, a
cone, 8 tori, 2 spheres, 361 B-spline surfaces and 506 surfaces of extrusion), as it lies there:
its keel on the plane z = 0, its bow toward +x (from x = −29.18 at the stern to x = 30.83 at the
bow), symmetric about y = 0, 60 long, 31 wide and 48 high. As the harness measures it, its volume
is 15551.00 and its area 9432.40. It is the same solid as the single-part STL printers are given
(its volume within 3e-5 of the STL's).

A **pass** is the original itself: a solid whose volume, area, centroid and inertia tensor (about
the centroid, of unit density) are the reference's, within 2e-5 rather than the usual 1e-6. The
file's own edges are loose: it declares an uncertainty of 0.001, carries no curves in its faces'
parameters, and some of its edges lie 0.006 off their faces, so two careful readings of it
disagree by up to 1.1e-5 (the harness's, by its edges' curves, and Open CASCADE's, by curves it
fits on the faces). The tolerance is twice that, so that a solid built as the original was built
is not failed by the file's slack. The centroid is graded by its distance over the reference's
radius of gyration (18.6), the inertia by its largest difference over its largest term.

Build it as nearly as you can, in as few lines as you can. Every number may be measured from the
reference (a section, a radius, a plane, a control net), but the implementation reads nothing at
run time: the solid is made by the kernel from the numbers written in its code, as every case's
is, and the reference's faces are not taken in as its own. Short of the original, a closed solid,
right side out, is **wrong**, and how near it came is shown beside it: its likeness to the
reference, the volume the two hold in common over the volume either holds (1 for the same solid),
and the lines of its implementation.

The likeness reads the reference's triangles, `benchy.stl.gz`: Open CASCADE's mesh of the STEP
file to a linear deflection of 0.01 (and an angular one of 0.5), closed, its volume within 3e-4 of
the solid's. Since the two are meshed apart, the reference itself, meshed as a result is, comes to
0.9996: that is a perfect match. A thin-walled boat is hard to be like, since a wall 1.2 thick
placed half a wall off shares half its volume. A hull lofted through its plans with the rest boxes
and cylinders comes to about 0.84; the reference's own hull net, planes and axes, with the deck
well still a loft, to about 0.96. The rest is in the fillets, the frames round the doors and
windows, the wheel, the chimney's taper and the lettering inside the wheelhouse's back wall.
