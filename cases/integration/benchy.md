+++
title = "#3DBenchy, as near as you can"
tags = ["integration", "part", "reference", "loft", "boolean", "cut", "union", "common"]
settled = false
reference = "benchy.stl.gz"
origin = "#3DBenchy by Creative Tools (Daniel Norée, 2015), CC0 1.0: Single-part/3DBenchy.stl of github.com/CreativeTools/3DBenchy at b542785"
+++

The tugboat that 3D printers are tested with: the canonical #3DBenchy, the single-part model as
its designers publish it for printing, 60 long, 31 wide and 48 high. It is a reference shape, not
a description: the solid is the one the triangles of `benchy.stl.gz` (beside this file) bound, as
they lie there, its keel on the plane z = 0, its bow toward +x (from x = −29.18 at the stern to
x = 30.83 at the bow), symmetric about y = 0.

Build it as nearly as you can, in as few lines as you can. Every number may be measured from the
reference (a section, a silhouette, a radius), but the implementation reads nothing at run time:
the solid is made by the kernel from the numbers written in its code, as every case's is, and it
may not be the reference's triangles taken in as faces. The hull is free-form, and nothing in the
model is a closed form, so no kernel's solid is the reference and the case has no answer to
grade against: a closed solid, right side out, is **built**.

What is shown beside each result, and not graded, is its likeness to the reference: the volume
the two hold in common over the volume either holds (1 for the same solid), and the lines of its
implementation. A thin-walled boat is hard to be like: a wall 1.2 thick placed half a wall off
shares half its volume. So a likeness of 0.85 is close, and the last few hundredths come from the
fillets and the small parts.

The reference's own measures, from its triangles: volume 15550.53 and area 9431.09. Its mesh is
not quite closed (1253 of its edges, welded where their corners are equal, are not matched by an
edge the other way), so the likeness reads it, as it reads every solid, by the winding number of
its faces and not by counting their crossings.
