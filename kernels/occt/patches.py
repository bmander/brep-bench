"""What the OCCT implementations of the patch cases share: the block under a Bézier patch whose top
face is trimmed on the whole patch, its walls upright and its floor at z = 0."""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakeSolid,
                                BRepBuilderAPI_Sewing)
from OCP.BRepLib import BRepLib
from OCP.Geom import Geom_BezierSurface
from OCP.TopAbs import TopAbs_SHELL
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.collections import Array2_gp_Pnt
from OCP.gp import gp_Pnt

from bench import Refused

TOLERANCE = 1e-9


def bezier(poles):
    """The Bézier surface whose control points are `poles[i][j]` (row i along u, column j along
    v): its degrees one less than the counts each way."""
    net = Array2_gp_Pnt(1, len(poles), 1, len(poles[0]))
    for i, row in enumerate(poles):
        for j, p in enumerate(row):
            net.SetValue(i + 1, j + 1, gp_Pnt(*p))
    return Geom_BezierSurface(net)


def poles_of(surface):
    """A Bézier surface's control points, row by row."""
    return [[surface.Pole(i, j).Coord() for j in range(1, surface.NbVPoles() + 1)]
            for i in range(1, surface.NbUPoles() + 1)]


def under(poles, us, vs):
    """The block under the part `us` × `vs` of the Bézier patch on `poles` (a graph over the plane
    z = 0): its top that part of the patch, the patch itself the face's surface (trimmed to its
    parameters there, not cut down to them); its floor in z = 0 under the top's rim; its four walls
    upright, each between an edge of the top and the floor. The six faces sewn into a solid."""
    patch = bezier(poles)
    top = BRepBuilderAPI_MakeFace(patch, us[0], us[1], vs[0], vs[1], TOLERANCE).Face()
    # the rim of the top, exactly: the patch cut to the part (its own surface, reparametrised), whose
    # outermost rows and columns of control points are the curves the walls stand on
    part = patch.Copy()
    part.Segment(us[0], us[1], vs[0], vs[1])
    rows = poles_of(part)
    floor = [[(x, y, 0.0) for (x, y, _) in row] for row in rows]
    rims = [rows[0], rows[-1], [r[0] for r in rows], [r[-1] for r in rows]]
    sewing = BRepBuilderAPI_Sewing(1e-7)
    sewing.Add(top)
    sewing.Add(BRepBuilderAPI_MakeFace(bezier(floor), TOLERANCE).Face())
    for rim in rims:
        wall = [[(x, y, 0.0), (x, y, z)] for (x, y, z) in rim]
        sewing.Add(BRepBuilderAPI_MakeFace(bezier(wall), TOLERANCE).Face())
    sewing.Perform()
    shells = TopExp_Explorer(sewing.SewedShape(), TopAbs_SHELL)
    if not shells.More():
        raise Refused("BRepBuilderAPI_Sewing made no shell of the block's faces")
    solid = BRepBuilderAPI_MakeSolid(TopoDS.Shell(shells.Current()))
    if not solid.IsDone():
        raise Refused("BRepBuilderAPI_MakeSolid declined the block's sewn shell")
    solid = solid.Solid()
    BRepLib.OrientClosedSolid_s(solid)
    return solid


def beyond(origin, normal, along, reach=10.0):
    """A box standing on the plane through `origin` square to `normal`, on the side `normal`
    points to, `reach` deep and `reach` wide either way along `along` (in the plane) and across it:
    what lies past the plane, near the origin."""
    from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
    from OCP.gp import gp_Ax2, gp_Dir, gp_Vec
    n, a = gp_Dir(*normal), gp_Dir(*along)
    across = gp_Vec(n).Crossed(gp_Vec(a))
    corner = gp_Pnt(*origin).Translated(gp_Vec(a).Multiplied(-reach / 2).Added(across.Multiplied(-reach / 2)))
    return BRepPrimAPI_MakeBox(gp_Ax2(corner, n, a), reach, reach, reach).Shape()
