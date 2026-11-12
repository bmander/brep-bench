+++
title = "#3DBenchy, as near as you can"
tags = ["integration", "part", "reference", "loft", "boolean", "cut", "union", "common"]
settled = false
reference = "benchy.step.gz"
reference_mesh = "benchy.stl.gz"
origin = "#3DBenchy by Daniel Norée (2015), CC0 1.0: its original CAD file as released in STEP for its tenth anniversary, the FreeCAD export '3DBenchy by Daniel Norée - CC0 1.0 Universal' of 2025-01-31 (printables.com/model/1263390, 3DBenchyCAD.step, SHA-256 ff85d227…)"
+++

The tugboat that 3D printers are tested with: #3DBenchy, from its designer's own CAD file, now in
the public domain. It is a reference shape, not a description: the solid is the one in
`benchy.step.gz` (beside this file), one closed B-rep of 1385 faces (432 planes, 75 cylinders, a
cone, 8 tori, 2 spheres, 361 B-spline surfaces and 506 surfaces of extrusion), as it lies there:
its keel on the plane z = 0, its bow toward +x (from x = −29.18 at the stern to x = 30.83 at the
bow), symmetric about y = 0, 60 long, 31 wide and 48 high. As the harness measures it, its volume
is 15551.00 and its area 9432.40. It is the same solid as the single-part STL printers are given
(its volume within 3e-5 of the STL's).

Build it as nearly as you can, in as few lines as you can. Every number may be measured from the
reference (a section, a silhouette, a radius, a control net), but the implementation reads nothing
at run time: the solid is made by the kernel from the numbers written in its code, as every case's
is. The hull is free-form and nothing in the model is a closed form a few numbers give, so no
kernel's solid is the reference, and the case has no answer to grade against: a closed solid,
right side out, is **built**.

What is shown beside each result, and not graded, is its likeness to the reference: the volume
the two hold in common over the volume either holds (1 for the same solid), and the lines of its
implementation. The likeness reads the reference's triangles, `benchy.stl.gz`: Open CASCADE's mesh
of the STEP file to a linear deflection of 0.01 (and an angular one of 0.5), closed, its volume
within 3e-4 of the solid's. Since the two are meshed apart, the reference itself, meshed as a
result is, comes to 0.9996: that is a perfect match. A thin-walled boat is hard to be like, since
a wall 1.2 thick placed half a wall off shares half its volume. A hull lofted through its plans
with the rest boxes and cylinders comes to about 0.84; the reference's own hull net, planes and
axes, with the deck well still a loft, to about 0.96. The rest is in the fillets, the frames round
the doors and windows, the wheel and the lettering.
