"""A rod grooved by a wheel slid along it. A rod, the cylinder of radius 5 about the z axis from
z = 0 to z = 20, grooved along its length by a wheel slid past it. The wheel is a torus: its axis
is parallel to the y axis, the circle through its tube's centres has radius 6, and its tube has
radius 2. It starts with its centre at (11.5, 0, −9), so the part of its tube nearest the rod is
centred 5.5 from the rod's axis and reaches 1.5 into it, and the whole wheel is below the rod. It
slides without turning along the z axis, up to centre (11.5, 0, 29), where the whole wheel is
above the rod. Cut from the rod everything the wheel passes through."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse, BRepAlgoAPI_Splitter
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakePrism, BRepPrimAPI_MakeTorus
from OCP.TopAbs import TopAbs_SHELL, TopAbs_SOLID
from OCP.TopExp import TopExp_Explorer
from OCP.collections import List_TopoDS_Shape
from OCP.gp import gp, gp_Ax2, gp_Ax3, gp_Cylinder, gp_Pln, gp_Pnt, gp_Trsf, gp_Vec

from bench import Refused

ROD_RADIUS = 5
ROD_LENGTH = 20
WHEEL_START = (11.5, 0, -9)
WHEEL_END = (11.5, 0, 29)
WHEEL_RADIUS = 6  # of the circle through the tube's centres
TUBE_RADIUS = 2


def shapes(*items):
    listed = List_TopoDS_Shape()
    for item in items:
        listed.Append(item)
    return listed


def build():
    centre = gp_Pnt(*WHEEL_START)
    wheel = BRepPrimAPI_MakeTorus(gp_Ax2(centre, gp.DY_s()), WHEEL_RADIUS, TUBE_RADIUS).Shape()
    slide = gp_Vec(centre, gp_Pnt(*WHEEL_END))
    moved = gp_Trsf()
    moved.SetTranslation(slide)
    wheel_at_end = BRepBuilderAPI_Transform(wheel, moved, True).Shape()

    # workaround: no sweep of a solid
    # OCCT's prism sweeps no solid ("Solids are not Processed"), but it sweeps a shell, each face
    # to the solid that face passes through. A face swept whole is only what it passes through if
    # the slide never runs along it inside it. The torus's normal, at the point of its tube's
    # circle turned u about the wheel's axis and v round the tube, is
    # cos v·(cos u·x + sin u·z) + sin v·y; it is square to the slide (z) where sin u = 0, the tube's
    # two circles in the plane z = −9 through the centre, and where cos v = 0, the wheel's two
    # circles of radius 6 on its tube's top and bottom (y = ±2). Split the torus along those four
    # circles, by that plane and by the cylinder of radius 6 about the wheel's axis; each piece
    # then slides without folding over itself.
    reach = 2 * (WHEEL_RADIUS + TUBE_RADIUS)  # the splitting faces run well past the wheel
    plane = BRepBuilderAPI_MakeFace(gp_Pln(centre, gp.DZ_s()), -reach, reach, -reach, reach).Face()
    cylinder = BRepBuilderAPI_MakeFace(gp_Cylinder(gp_Ax3(centre, gp.DY_s()), WHEEL_RADIUS),
                                       0, 2 * math.pi, -reach, reach).Face()
    splitter = BRepAlgoAPI_Splitter()
    splitter.SetArguments(shapes(TopExp_Explorer(wheel, TopAbs_SHELL).Current()))
    splitter.SetTools(shapes(plane, cylinder))
    splitter.Build()
    if not splitter.IsDone():
        raise Refused("BRepAlgoAPI_Splitter could not split the wheel along its outline")
    prism = BRepPrimAPI_MakePrism(splitter.Shape(), slide)
    if not prism.IsDone():
        raise Refused("BRepPrimAPI_MakePrism could not slide the wheel's faces")

    # Everything the wheel passes through is the wheel where it starts, the wheel where it ends,
    # and everything its faces pass through: a point the wheel covers that no face ever crosses
    # stays inside the wheel all the way.
    tools = shapes(wheel_at_end)
    swept = TopExp_Explorer(prism.Shape(), TopAbs_SOLID)
    while swept.More():
        tools.Append(swept.Current())
        swept.Next()
    fuse = BRepAlgoAPI_Fuse()
    fuse.SetArguments(shapes(wheel))
    fuse.SetTools(tools)
    fuse.Build()
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the faces' sweeps and the wheel at each end")

    rod = BRepPrimAPI_MakeCylinder(gp.XOY_s(), ROD_RADIUS, ROD_LENGTH).Shape()
    cut = BRepAlgoAPI_Cut(rod, fuse.Shape())
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the groove")
    return cut.Shape()
