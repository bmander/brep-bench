"""A regular tetrahedron grown by 1. The tetrahedron with corners (3, 3, 3), (3, −3, −3),
(−3, 3, −3) and (−3, −3, 3), grown by 1."""
from itertools import combinations

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon,
                                BRepBuilderAPI_MakeSolid, BRepBuilderAPI_Sewing)
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Pnt

from bench import Refused
from offset import offset

CORNERS = [(3, 3, 3), (3, -3, -3), (-3, 3, -3), (-3, -3, 3)]
GROWTH = 1


def build():
    # Each three of the corners make a face; sewn, the four faces close into a shell.
    sewing = BRepBuilderAPI_Sewing()
    for triangle in combinations(CORNERS, 3):
        polygon = BRepBuilderAPI_MakePolygon(*(gp_Pnt(*p) for p in triangle), True)
        sewing.Add(BRepBuilderAPI_MakeFace(polygon.Wire()).Face())
    sewing.Perform()
    solid = BRepBuilderAPI_MakeSolid(TopoDS.Shell(sewing.SewedShape()))
    if not solid.IsDone():
        raise Refused("BRepBuilderAPI_MakeSolid could not close the tetrahedron")
    return offset(solid.Shape(), GROWTH)
