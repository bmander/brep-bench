"""#3DBenchy, as near as a few solids come to it, from numbers read off the reference.

The hull's side is the reference's own surface, its 12 × 6 bicubic net written out (rounded to
0.01), and its mirror image: sewn with a ruled stern between their ends and the planes z = 0 and
z = 30 into a solid, kept below the sheer (an arc of radius 103.08 aft, the gunwale's plane at the
bow). The deck well is a ruled loft through its plan at four heights, read on thirteen rays from
the origin, kept above the deck. The wheelhouse, its room, the roof and the cargo box are each the
common part of the half-spaces under the reference's own planes (the walls leaning 1.5°, 4.5° and
3.2°, the roof 5.5°, the box 5°); the doors, the windows, the chimney, the rod holder, the drain
and the hawse pipes are cylinders on the reference's own axes.
"""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakePolygon, BRepBuilderAPI_MakeSolid,
                                BRepBuilderAPI_MakeWire, BRepBuilderAPI_Sewing)
from OCP.BRepFill import BRepFill
from OCP.BRepLib import BRepLib
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakePrism
from OCP.GC import GC_MakeArcOfCircle, GC_MakeSegment
from OCP.Geom import Geom_BSplineSurface
from OCP.TopAbs import TopAbs_SHELL
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.collections import Array1_double, Array1_int, Array2_gp_Pnt
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt, gp_Vec

from bench import Refused

# the hull's side: its net's rows from the stern to the stem, each the x and y of a pole at each
# of the heights Z
Z = [0, 3.31, 9.86, 20, 26.67, 30]
NET = [[-25, 6.5, -26.58, 8.2, -29.7, 11.55, -30.07, 13.11, -30.02, 13.7, -30, 14],
       [-16.67, 7, -18.03, 8.92, -20.73, 12.71, -19.33, 13.99, -19.33, 14.22, -19.33, 14.33],
       [-6.95, 7.58, -9.03, 9.65, -13.14, 13.72, -9.06, 14.83, -9.1, 14.71, -9.12, 14.65],
       [1.7, 8.1, 1.04, 10.53, -0.27, 15.33, 4.9, 15.97, 4.78, 15.38, 4.72, 15.09],
       [3.27, 7.91, 3.01, 10.4, 2.47, 15.32, 7.73, 15.81, 7.59, 15.16, 7.52, 14.84],
       [6.12, 7.14, 6.78, 9.59, 8.06, 14.43, 13.32, 14.65, 13.12, 13.96, 13.02, 13.62],
       [7.4, 6.57, 8.58, 8.92, 10.9, 13.55, 16.07, 13.65, 15.85, 12.99, 15.74, 12.65],
       [9.71, 5.17, 11.9, 7.09, 16.24, 10.9, 21.18, 10.88, 20.96, 10.32, 20.85, 10.04],
       [10.74, 4.32, 13.42, 5.94, 18.74, 9.14, 23.54, 9.1, 23.35, 8.63, 23.25, 8.4],
       [12.56, 2.38, 16.08, 3.24, 23.04, 4.96, 27.63, 4.93, 27.57, 4.69, 27.54, 4.56],
       [13.35, 1.27, 17.21, 1.7, 24.84, 2.54, 29.35, 2.55, 29.4, 2.43, 29.43, 2.37],
       [14, 0, 18.1, 0, 26.21, 0, 30.68, 0, 30.89, 0, 31, 0]]
UKNOTS = [(0, 4), (0.5052, 2), (0.6289, 2), (0.7526, 2), (0.8763, 2), (1, 4)]  # (knot, times)
VKNOTS = [(0, 4), (0.3389, 1), (0.6699, 1), (1, 4)]
# the deck well's plan at each height, its half on y ≥ 0 from the stem round to the stern
BOW = [(27.77, 0), (26.11, 3.67), (21.05, 7.66), (13.93, 11.69)]
AFT = [(-17.78, 10.26), (-25.8, 9.39), (-27.06, 4.77)]
WELL = {8.6: BOW + [(7.41, 10.59), (3.97, 10.91), (0.97, 11.08), (-1.92, 10.89), (-8.54, 10.18),
                    (-17.78, 10.26), (-23.57, 8.58), (-26.17, 4.62), (-27.04, 0)],
        10.5: BOW + [(7.41, 10.59), (4.37, 12.02), (1.07, 12.2), (-2.11, 11.97), (-9.38, 11.18)]
              + AFT + [(-27.6, 0)],
        12: BOW + [(8.22, 11.74), (4.57, 12.56), (1.11, 12.72), (-2.2, 12.45), (-9.38, 11.18)]
            + AFT + [(-28.04, 0)],
        28: BOW + [(9.54, 13.63), (4.57, 12.56), (1.11, 12.72), (-2.2, 12.45), (-9.38, 11.18)]
            + AFT + [(-28.04, 0)]}
DECK = [(-40, 8.5), (5.95, 8.5), (10.12, 14.46), (40, 23.42), (40, 60), (-40, 60)]  # (x, z)
SHEER = (-23.0812, 113.7248, 103.0821)  # the aft sheer's circle in (x, z): its centre, its radius
STERN = -29.1765
# the sines and cosines of the walls' leans: the wheelhouse's back 1.5°, its front 4.5°, the roof
# 5.5° and the cargo box 5°
S15, C15, S45, C45 = 0.02618, 0.99966, 0.07846, 0.99692
S55, C55, S5, C5 = 0.09585, 0.99540, 0.08716, 0.99619


def op(kind, a, *bs):
    for b in bs:
        made = kind(a, b)
        if not made.IsDone():
            raise Refused(f"{kind.__name__} declined")
        a = made.Shape()
    return a


def half(a, b, c, d):
    """The half-space a·x + b·y + c·z ≤ d: a cube 400 across whose top face lies on its plane."""
    k = math.sqrt(a * a + b * b + c * c)
    n, d = gp_Dir(a, b, c), d / k
    axes = gp_Ax2(gp_Pnt(a * d / k, b * d / k, c * d / k), n)
    corner = gp_Pnt(*(axes.Location().Coord(i) - 200 * (axes.XDirection().Coord(i)
                      + axes.YDirection().Coord(i)) - 400 * n.Coord(i) for i in (1, 2, 3)))
    return BRepPrimAPI_MakeBox(gp_Ax2(corner, n, axes.XDirection()), 400, 400, 400).Shape()


def convex(*planes):
    return op(BRepAlgoAPI_Common, *[half(*p) for p in planes])


def cyl(p, axis, r, h):
    return BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(*p), gp_Dir(*axis)), r, h).Shape()


def box(lo, hi):
    return BRepPrimAPI_MakeBox(gp_Pnt(*lo), gp_Pnt(*hi)).Shape()


def ring(half, z):
    """The closed polygon in the plane at height z of a half plan and its mirror image."""
    poly = BRepBuilderAPI_MakePolygon()
    for x, y in [(x, -y) for x, y in half[:-1]] + half[:0:-1]:
        poly.Add(gp_Pnt(x, y, z))
    poly.Close()
    return poly.Wire()


def across(pieces):
    """The prism through y = ±20 of an outline in the xz plane: lines, and arcs through three
    points."""
    wire = BRepBuilderAPI_MakeWire()
    for piece in pieces:
        pts = [gp_Pnt(x, -20, z) for x, z in piece]
        curve = GC_MakeSegment(*pts) if len(pts) == 2 else GC_MakeArcOfCircle(*pts)
        wire.Add(BRepBuilderAPI_MakeEdge(curve.Value()).Edge())
    face = BRepBuilderAPI_MakeFace(wire.Wire()).Face()
    return BRepPrimAPI_MakePrism(face, gp_Vec(0, 40, 0)).Shape()


def hull():
    """The side net and its mirror image, sewn with the stern and the planes z = 0 and z = 30."""
    def side(s):
        poles = Array2_gp_Pnt(1, 12, 1, 6)
        for i, row in enumerate(NET):
            for j in range(6):
                poles.SetValue(i + 1, j + 1, gp_Pnt(row[2 * j], s * row[2 * j + 1], Z[j]))
        knots = []
        for kms in (UKNOTS, VKNOTS):
            k, m = Array1_double(1, len(kms)), Array1_int(1, len(kms))
            for i, (v, mult) in enumerate(kms):
                k.SetValue(i + 1, v)
                m.SetValue(i + 1, mult)
            knots += [k, m]
        return Geom_BSplineSurface(poles, knots[0], knots[2], knots[1], knots[3], 3, 3)
    port, starboard = side(1), side(-1)
    edge = lambda curve: BRepBuilderAPI_MakeEdge(curve).Edge()  # noqa: E731
    sewing = BRepBuilderAPI_Sewing(1e-5)
    for s in (port, starboard):
        sewing.Add(BRepBuilderAPI_MakeFace(s, 1e-7).Face())
    sewing.Add(BRepFill.Face_s(edge(port.UIso(0)), edge(starboard.UIso(0))))
    for v in (0, 1):
        cap = BRepBuilderAPI_MakeWire(edge(port.VIso(v)), edge(starboard.VIso(v)),
                                      BRepBuilderAPI_MakeEdge(port.Value(0, v),
                                                              starboard.Value(0, v)).Edge())
        sewing.Add(BRepBuilderAPI_MakeFace(cap.Wire(), True).Face())
    sewing.Perform()
    shells = TopExp_Explorer(sewing.SewedShape(), TopAbs_SHELL)
    if not shells.More():
        raise Refused("BRepBuilderAPI_Sewing made no shell of the hull's faces")
    solid = BRepBuilderAPI_MakeSolid(TopoDS.Shell(shells.Current())).Solid()
    BRepLib.OrientClosedSolid_s(solid)
    return solid


def build():
    cx, cz, r = SHEER
    sheer = [(x, cz - math.sqrt(r * r - (x - cx) ** 2)) for x in (4.16, -12, STERN)]
    side = across([[(STERN, -1), (32, -1)], [(32, -1), (32, 27.89)], [(32, 27.89), (10.22, 21.36)],
                   [(10.22, 21.36), sheer[0]], sheer, [sheer[2], (STERN, -1)]])
    well = BRepOffsetAPI_ThruSections(True, True)
    for z, plan in WELL.items():
        well.AddWire(ring(plan, z))
    deck = across([[a, b] for a, b in zip(DECK, DECK[1:] + DECK[:1])])
    boat = op(BRepAlgoAPI_Cut, op(BRepAlgoAPI_Common, hull(), side),
              op(BRepAlgoAPI_Common, well.Shape(), deck))
    # the wheelhouse: its walls, less its room and the room's upper part over the console
    walls = [(C15, 0, S15, -4.7751), (0.05545, -0.99812, 0.02618, -5.1525),
             (0.05545, 0.99812, 0.02618, -5.1525), (0, 0, -1, -36.614)]
    room = [(-a, -b, -c, -d) for a, b, c, d in walls]  # the walls' faces facing into the room
    house = op(BRepAlgoAPI_Cut,
               convex((-C15, 0, -S15, 6.7751), (C45, 0, -S45, 10.2992), (0, 0, -1, -8),
                      (-0.05545, 0.99812, -0.02618, 7.1525), (-0.05545, -0.99812, -0.02618, 7.1525),
                      (0, 0, 1, 36.614)),
               convex(*room, (C45, 0, -S45, 5.8069), (0, 0, -1, -8.5)),
               convex(*room, (C45, 0, -S45, 8.2992), (0, 0, -1, -20.5)))
    roof = convex((-S55, 0, C55, 38.7973), (C55, 0, S55, 17.7101), (-C55, 0, -S55, 5.2885),
                  (-0.05745, 0.99833, -0.00553, 8.8967), (-0.05745, -0.99833, -0.00553, 8.8967),
                  (S55, 0, -C55, -35.7973))
    cargo = convex((-C5, 0, -S5, 16.7295), (0, C5, -S5, 4.7752), (0, -C5, -S5, 4.7752),
                   (0, 0, 1, 15.5), (0, 0, -1, -8), (1, 0, 0, -6))
    rod = op(BRepAlgoAPI_Common, cyl((-29.939, 0, 22.073), (0.28046, 0, -0.95987), 1.75, 20),
             convex((0, 0, 1, 12.87), (0, 0, -1, -8)))
    up = (0, 0, 1)
    boat = op(BRepAlgoAPI_Fuse, boat, house, roof, cargo, rod, cyl((-4, 0, 36), up, 3, 9.3),
              cyl((-4, 0, 45.3), up, 3.5, 2.7))
    cuts = [box((-15.998, -3.998, 6.5), (-8.995, 3.998, 16)),  # the cargo box's hold
            cyl((-4, 0, 37), up, 1.5, 12),  # the chimney's bore
            box((9, -5.25, 24.611), (16, 5.25, 34.112)),  # the front window
            cyl((-10, 0, 27.1755), (C15, 0, S15), 4.5, 6),  # the rear window
            cyl((-24.8065, 0, 4.508), (-0.28046, 0, 0.95987), 1, 10),  # the rod holder's bore
            cyl((-26.5, 0, 1.6), (1, 0, 0), 1, 6.5)]  # the drain
    for s in (1, -1):
        cuts += [convex((-0.99812, -0.0561 * s, -0.02475, 1.3166),  # a door, below its arch
                        (0.99538, 0.0532 * s, -0.07997, 4.7812), (0, 0, -1, -8.5),
                        (0, 0, 1, 27.52), (0, -s, 0, -2), (0, s, 0, 9.5)),
                 cyl((2.392, 2.0057 * s, 27.5982), (-0.05545, s, -0.02618), 4.5, 8),  # its arch
                 cyl((20.6648, 1.0843 * s, 21.4209), (0.70711, 0.6984 * s, -0.11062), 2, 12)]
    return op(BRepAlgoAPI_Cut, boat, *cuts)
