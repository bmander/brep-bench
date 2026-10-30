"""A box slid aslant. The solid a box fills as it slides along a straight line without turning.
The box starts as the box from (0, 0, 0) to (10, 6, 4) and slides along the vector (8, 5, 3),
ending as the box from (8, 5, 3) to (18, 11, 7). Make the solid of everything the box passes
through.

It is a convex polyhedron with twelve faces, in six parallel pairs: the box's three pairs of
faces, and three pairs of parallelograms, each with one edge of the box and the slide's vector as
its sides."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakePrism
from OCP.TopAbs import TopAbs_SHELL, TopAbs_SOLID
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.collections import List_TopoDS_Shape
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused

CORNER = (0, 0, 0)
FAR_CORNER = (10, 6, 4)
SLIDE = (8, 5, 3)


def build():
    start = BRepPrimAPI_MakeBox(gp_Pnt(*CORNER), gp_Pnt(*FAR_CORNER)).Shape()
    end = BRepPrimAPI_MakeBox(gp_Pnt(*(c + d for c, d in zip(CORNER, SLIDE))),
                              gp_Pnt(*(c + d for c, d in zip(FAR_CORNER, SLIDE)))).Shape()

    # workaround: no sweep of a solid
    # OCCT's prism sweeps no solid ("Solids are not Processed"), but it sweeps a shell: each of
    # its faces to the solid that face passes through, the six together a CompSolid. A flat face
    # slid along a vector out of its plane never folds over itself, so each of those solids is
    # exactly what its face passes through.
    shell = TopoDS.Shell(TopExp_Explorer(start, TopAbs_SHELL).Current())
    prism = BRepPrimAPI_MakePrism(shell, gp_Vec(*SLIDE))
    if not prism.IsDone():
        raise Refused("BRepPrimAPI_MakePrism could not slide the box's faces")

    # Everything the box passes through is the box where it starts, the box where it ends, and
    # everything its faces pass through: a point the box covers at some instant that no face
    # ever crosses stays inside the box all the way, so it is in the box at both ends.
    tools = List_TopoDS_Shape()
    swept = TopExp_Explorer(prism.Shape(), TopAbs_SOLID)
    while swept.More():
        tools.Append(swept.Current())
        swept.Next()
    tools.Append(end)
    arguments = List_TopoDS_Shape()
    arguments.Append(start)
    fuse = BRepAlgoAPI_Fuse()
    fuse.SetArguments(arguments)
    fuse.SetTools(tools)
    fuse.Build()
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the faces' sweeps and the box at each end")
    return fuse.Shape()
