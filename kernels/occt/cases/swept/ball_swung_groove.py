"""A groove swung by a ball. A block, the box from (−6, −6, −8) to (18, 18, 0), with a curved
groove cut into its top face by a ball. The ball has radius 3 and its centre starts at (12, 0, 0),
on the top face, so it is half sunk in the block. It swings a quarter turn about the z axis,
right-handed (from +x towards +y), so that its centre ends at (0, 12, 0); its centre stays in the
top face all the way. Cut from the block everything the ball passes through.

The groove is a round-bottomed channel: a quarter of a torus's tube below the top face, closed at
each end by a quarter of the ball."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse, BRepAlgoAPI_Splitter
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeRevol, BRepPrimAPI_MakeSphere
from OCP.TopAbs import TopAbs_SHELL, TopAbs_SOLID
from OCP.TopExp import TopExp_Explorer
from OCP.collections import List_TopoDS_Shape
from OCP.gp import gp, gp_Ax2, gp_Pln, gp_Pnt

from bench import Refused

BLOCK_LOW, BLOCK_HIGH = (-6, -6, -8), (18, 18, 0)
BALL_RADIUS = 3
SWING_RADIUS = 12  # the centre's distance from the z axis
SWING = 90  # degrees, about the z axis


def shapes(*items):
    listed = List_TopoDS_Shape()
    for item in items:
        listed.Append(item)
    return listed


def build():
    start = gp_Pnt(SWING_RADIUS, 0, 0)
    end = gp_Pnt(SWING_RADIUS * math.cos(math.radians(SWING)),
                 SWING_RADIUS * math.sin(math.radians(SWING)), 0)
    # The ball's poles on the y axis, so its equator lies in the plane y = 0.
    ball = BRepPrimAPI_MakeSphere(gp_Ax2(start, gp.DY_s()), BALL_RADIUS).Shape()
    ball_at_end = BRepPrimAPI_MakeSphere(end, BALL_RADIUS).Shape()

    # OCCT's revolution sweeps no solid ("Solids are not Processed"), but it sweeps a shell, each
    # face to the solid that face passes through. A face swept whole is only what it passes
    # through if the motion never runs along it inside it: a point p of the ball moves along
    # z × p, which runs along the sphere where (z × p)·(p − c) = 0, that is (z × c)·(p − c) = 0
    # for the centre c = (12, 0, 0): the great circle in the plane y = 0, through the axis and
    # the centre. So split the sphere there, into the half facing the turn and the half facing
    # away; each half then sweeps without folding, and that circle sweeps to the torus's tube.
    reach = 2 * SWING_RADIUS  # the splitting face, a square well past the ball
    plane = BRepBuilderAPI_MakeFace(gp_Pln(gp.Origin_s(), gp.DY_s()),
                                    -reach, reach, -reach, reach).Face()
    splitter = BRepAlgoAPI_Splitter()
    splitter.SetArguments(shapes(TopExp_Explorer(ball, TopAbs_SHELL).Current()))
    splitter.SetTools(shapes(plane))
    splitter.Build()
    if not splitter.IsDone():
        raise Refused("BRepAlgoAPI_Splitter could not split the ball along its equator")
    revol = BRepPrimAPI_MakeRevol(splitter.Shape(), gp.OZ_s(), math.radians(SWING))
    if not revol.IsDone():
        raise Refused("BRepPrimAPI_MakeRevol could not swing the ball's faces")

    # Everything the ball passes through is the ball where it starts, the ball where it ends, and
    # everything its faces pass through: a point the ball covers that no face ever crosses stays
    # inside the ball all the way.
    tools = shapes(ball_at_end)
    swept = TopExp_Explorer(revol.Shape(), TopAbs_SOLID)
    while swept.More():
        tools.Append(swept.Current())
        swept.Next()
    fuse = BRepAlgoAPI_Fuse()
    fuse.SetArguments(shapes(ball))
    fuse.SetTools(tools)
    fuse.Build()
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the faces' sweeps and the ball at each end")

    block = BRepPrimAPI_MakeBox(gp_Pnt(*BLOCK_LOW), gp_Pnt(*BLOCK_HIGH)).Shape()
    cut = BRepAlgoAPI_Cut(block, fuse.Shape())
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the groove")
    return cut.Shape()
