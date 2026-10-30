"""A rod grooved by a wheel of two beads. A rod, the cylinder of radius 5 about the z axis from
z = 0 to z = 20, grooved along its length by a wheel of two beads slid past it. The beads are two
tori on one axis, which runs parallel to the y axis through (11.5, 0, −9):

- the first bead's centre is (11.5, 0, −9); the circle through its tube's centres has radius 6,
  and its tube has radius 2;
- the second bead's centre is (11.5, 1.5, −9), 1.5 further along the axis; the circle through its
  tube's centres also has radius 6, and its tube has radius 1.5.

The two beads overlap, and the wheel is their union. It starts below the rod, as placed, and
slides without turning 40 along the z axis, up to where it is wholly above the rod. Cut from the
rod everything the wheel passes through."""
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
FIRST_CENTRE, FIRST_TUBE = (11.5, 0, -9), 2
SECOND_CENTRE, SECOND_TUBE = (11.5, 1.5, -9), 1.5
WHEEL_RADIUS = 6  # of each bead's circle through its tube's centres
SLIDE = (0, 0, 40)


def shapes(*items):
    listed = List_TopoDS_Shape()
    for item in items:
        listed.Append(item)
    return listed


def fused(arguments, tools, what):
    fuse = BRepAlgoAPI_Fuse()
    fuse.SetArguments(shapes(*arguments))
    fuse.SetTools(shapes(*tools))
    fuse.Build()
    if not fuse.IsDone():
        raise Refused(f"BRepAlgoAPI_Fuse declined {what}")
    return fuse.Shape()


def slid(bead, centre, tube):
    """Everything one bead passes through as it slides."""
    moved = gp_Trsf()
    moved.SetTranslation(gp_Vec(*SLIDE))
    bead_at_end = BRepBuilderAPI_Transform(bead, moved, True).Shape()

    # workaround: no sweep of a solid
    # OCCT's prism sweeps no solid ("Solids are not Processed"), but it sweeps a shell, each face
    # to the solid that face passes through. A face swept whole is only what it passes through if
    # the slide never runs along it inside it. A bead's normal, at the point of its tube's circle
    # turned u about the axis and v round the tube, is cos v·(cos u·x + sin u·z) + sin v·y; it is
    # square to the slide (z) where sin u = 0, the tube's two circles in the plane z = −9 through
    # the axis, and where cos v = 0, the bead's circles of radius 6 on its tube's top and bottom.
    # Split the bead along those four circles, by that plane and by the cylinder of radius 6
    # about the axis; each piece then slides without folding over itself.
    reach = 2 * (WHEEL_RADIUS + tube)  # the splitting faces run well past the bead
    plane = BRepBuilderAPI_MakeFace(gp_Pln(centre, gp.DZ_s()), -reach, reach, -reach, reach).Face()
    cylinder = BRepBuilderAPI_MakeFace(gp_Cylinder(gp_Ax3(centre, gp.DY_s()), WHEEL_RADIUS),
                                       0, 2 * math.pi, -reach, reach).Face()
    splitter = BRepAlgoAPI_Splitter()
    splitter.SetArguments(shapes(TopExp_Explorer(bead, TopAbs_SHELL).Current()))
    splitter.SetTools(shapes(plane, cylinder))
    splitter.Build()
    if not splitter.IsDone():
        raise Refused("BRepAlgoAPI_Splitter could not split a bead along its outline")
    prism = BRepPrimAPI_MakePrism(splitter.Shape(), gp_Vec(*SLIDE))
    if not prism.IsDone():
        raise Refused("BRepPrimAPI_MakePrism could not slide a bead's faces")

    # Everything the bead passes through is the bead where it starts, the bead where it ends, and
    # everything its faces pass through: a point the bead covers that no face ever crosses stays
    # inside the bead all the way.
    pieces = []
    swept = TopExp_Explorer(prism.Shape(), TopAbs_SOLID)
    while swept.More():
        pieces.append(swept.Current())
        swept.Next()
    return fused([bead], pieces + [bead_at_end], "a bead's faces' sweeps and the bead at each end")


def build():
    # The wheel passes through a point when one of its beads does, so what the wheel passes
    # through is the union of what each bead passes through. (Swept as one solid, split the same
    # way along its outline, the union's faces' sweeps defeat OCCT's fuse: it returns six solids
    # that BRepCheck finds invalid.)
    sweeps = []
    for centre, tube in [(FIRST_CENTRE, FIRST_TUBE), (SECOND_CENTRE, SECOND_TUBE)]:
        centre = gp_Pnt(*centre)
        bead = BRepPrimAPI_MakeTorus(gp_Ax2(centre, gp.DY_s()), WHEEL_RADIUS, tube).Shape()
        sweeps.append(slid(bead, centre, tube))
    wheel_sweep = fused(sweeps[:1], sweeps[1:], "the two beads' sweeps")

    rod = BRepPrimAPI_MakeCylinder(gp.XOY_s(), ROD_RADIUS, ROD_LENGTH).Shape()
    cut = BRepAlgoAPI_Cut(rod, wheel_sweep)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the groove")
    return cut.Shape()
