"""A V-belt pulley. A pulley turned about the z axis, symmetric about the plane z = 0:

- a hub, a cylinder of radius 12 from z = −14 to z = 14;
- a rim, a disc of radius 28 from z = −9 to z = 9, so the hub stands 5 proud of the rim on each
  side;
- a bore of radius 6 on the z axis, right through, from z = −14 to z = 14.

A V-shaped belt groove runs all the way round the rim. In the xz plane (the first number is x and
the second z) the groove is the part of the triangle with corners (22, 0), (29, −5) and (29, 5)
that lies inside the pulley, turned a full turn about the z axis and taken away. So the groove's
root is the circle of radius 22 at z = 0, 6 deep below the rim's face, and its two flanks are cones
that open 5 in z for every 7 outward in radius, meeting the rim's face (radius 28) at
z = ±30/7. The triangle reaches 1 past the rim's face so the groove opens cleanly; if you cut it
with a revolved triangle, keep it reaching past the rim like that."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Pnt

from bench import Refused

BORE, HUB, RIM = 6, 12, 28        # radii
HUB_HALF, RIM_HALF = 14, 9        # half-lengths along z
GROOVE = [(22, 0), (29, -5), (29, 5)]


def revolved(outline):
    """The solid swept by a polygon in the xz plane turned a full turn about the z axis."""
    polygon = BRepBuilderAPI_MakePolygon()
    for x, z in outline:
        polygon.Add(gp_Pnt(x, 0, z))
    polygon.Close()
    section = BRepBuilderAPI_MakeFace(polygon.Wire(), True).Face()
    revol = BRepPrimAPI_MakeRevol(section, gp.OZ_s())
    if not revol.IsDone():
        raise Refused("BRepPrimAPI_MakeRevol declined a section")
    return revol.Shape()


def build():
    # The pulley's half-section: the bore's line, the hub's ends and sides, the rim's faces.
    body = revolved([(BORE, -HUB_HALF), (HUB, -HUB_HALF), (HUB, -RIM_HALF), (RIM, -RIM_HALF),
                     (RIM, RIM_HALF), (HUB, RIM_HALF), (HUB, HUB_HALF), (BORE, HUB_HALF)])
    cut = BRepAlgoAPI_Cut(body, revolved(GROOVE))
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the groove")
    return cut.Shape()
