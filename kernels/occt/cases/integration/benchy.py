"""#3DBenchy, as near as a few solids come to it. The hull is lofted (ruled) through its plan at
seven heights, each read from the reference on eleven rays from the origin, and kept below its
side profile; its deck well is the same loft with each plan moved in by the wall, kept above the
deck. The wheelhouse is a box less its room, under a roof sloping 1 in 10, with a console at its
front and a sill at its back; its doors are a box and a cylinder through both sides, its front
window a box, its rear window a cylinder. The chimney and the rod holder are bored cylinders, the
cargo box a box less its hold. Every number was measured from the reference's sections."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakeOffset, BRepOffsetAPI_ThruSections
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakePrism
from OCP.GeomAbs import GeomAbs_Intersection
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt, gp_Vec

from bench import Refused

# the hull's plan at each height, its half on y ≥ 0 from the bow round to the stern
PLAN = {0: [(13.9, 0), (13.1, 1.8), (11.1, 4.0), (7.7, 6.5), (2.9, 7.9), (-1.4, 7.9), (-6.4, 7.6),
            (-12.8, 7.4), (-18.9, 6.9), (-25.1, 4.4), (-25.1, 0)],
        2: [(16.1, 0), (15.0, 2.1), (12.8, 4.6), (8.8, 7.4), (3.3, 9.1), (-1.6, 9.1), (-7.5, 8.9),
            (-14.5, 8.4), (-21.5, 7.8), (-25.9, 4.6), (-25.1, 0)],
        5: [(19.6, 0), (18.3, 2.6), (15.5, 5.7), (10.8, 9.1), (4.1, 11.1), (-2.0, 11.2),
            (-8.9, 10.6), (-17.1, 9.9), (-25.1, 9.1), (-27.1, 4.8), (-27.1, 0)],
        8.5: [(23.4, 0), (21.6, 3.0), (18.4, 6.7), (12.7, 10.6), (4.8, 13.1), (-2.3, 13.1),
              (-10.4, 12.4), (-19.7, 11.4), (-28.0, 10.2), (-28.6, 5.1), (-28.6, 0)],
        12: [(26.1, 0), (24.3, 3.4), (20.6, 7.5), (14.2, 11.9), (5.3, 14.6), (-2.6, 14.6),
             (-11.0, 13.1), (-20.6, 11.9), (-28.7, 10.4), (-29.1, 5.1), (-29.1, 0)],
        17: [(28.9, 0), (26.7, 3.8), (22.5, 8.2), (15.4, 12.9), (5.6, 15.4), (-2.6, 14.6),
             (-11.0, 13.1), (-20.6, 11.9), (-28.7, 10.4), (-29.1, 5.1), (-29.1, 0)],
        28: [(30.9, 0), (28.4, 4.0), (23.8, 8.7), (16.0, 13.4), (5.6, 15.4), (-2.6, 14.6),
             (-11.0, 13.1), (-20.6, 11.9), (-28.7, 10.4), (-29.1, 5.1), (-29.1, 0)]}
# the side profile (x, z): the sheer, the stern and the keel, the bow
SIDE = [(30.9, 25.9), (28.8, 27.0), (9.8, 21.2), (5.4, 15.1), (2.5, 13.8), (-9.5, 11.5),
        (-24.7, 10.8), (-28.9, 11.9), (-29.1, 9.1), (-25.0, 0.0), (13.8, 0.0), (26.9, 12.9)]
DECK = [(-40, 8.5), (5, 8.5), (10.5, 15), (40, 23.2), (40, 60), (-40, 60)]  # (x, z), the well above


def op(kind, a, *bs):
    for b in bs:
        made = kind(a, b)
        if not made.IsDone():
            raise Refused(f"{kind.__name__} declined")
        a = made.Shape()
    return a


def wire(pts, ax, w):
    """The closed polygon `pts` in the plane square to axis `ax` at `w` along it."""
    poly = BRepBuilderAPI_MakePolygon()
    for u, v in pts:
        p = [u, v]
        p.insert(ax, w)
        poly.Add(gp_Pnt(*p))
    poly.Close()
    return poly.Wire()


def prism(pts, ax, a, b):
    d = [0, 0, 0]
    d[ax] = b - a
    face = BRepBuilderAPI_MakeFace(wire(pts, ax, a)).Face()
    return BRepPrimAPI_MakePrism(face, gp_Vec(*d)).Shape()


def box(lo, hi):
    return BRepPrimAPI_MakeBox(gp_Pnt(*lo), gp_Pnt(*hi)).Shape()


def cyl(p, axis, r, h):
    return BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(*p), gp_Dir(*axis)), r, h).Shape()


def lofted(inset):
    """The hull's plans lofted, each moved in by `inset`."""
    loft = BRepOffsetAPI_ThruSections(True, True)
    for z, half in PLAN.items():
        w = wire([(x, -y) for x, y in half[:-1]] + [(x, y) for x, y in half[:0:-1]], 2, z)
        if inset:
            moved = BRepOffsetAPI_MakeOffset(w, GeomAbs_Intersection)
            moved.Perform(-inset)
            w = TopoDS.Wire(moved.Shape())
        loft.AddWire(w)
    return loft.Shape()


def build():
    up, across, along = (0, 0, 1), (0, 1, 0), (1, 0, 0)
    hull = op(BRepAlgoAPI_Common, lofted(0), prism(SIDE, 1, -20, 20))
    boat = op(BRepAlgoAPI_Cut, hull, op(BRepAlgoAPI_Common, lofted(1.3), prism(DECK, 1, -20, 20)))
    house = op(BRepAlgoAPI_Cut, box((-7.25, -8.3, 8), (12.5, 8.3, 36.6)),
               box((-5.5, -6.5, 8.5), (11, 6.5, 37)))
    roof = op(BRepAlgoAPI_Common, box((-9, -9, 36.5), (14, 9, 45)),
              prism([(-20, 0), (40, 0), (40, 42.9), (-20, 36.9)], 1, -20, 20))
    stack = op(BRepAlgoAPI_Cut, cyl((-4.2, 0, 36.6), up, 2.9, 11.4),
               cyl((-4.2, 0, 36.9), up, 1.5, 12))
    rod = op(BRepAlgoAPI_Cut, cyl((-26.5, 0, 8), up, 1.6, 4.3), cyl((-26.5, 0, 9), up, 1.1, 4))
    console, sill = box((6.8, -6.5, 8), (12.5, 6.5, 20.5)), box((-9, -6.5, 8), (-5.5, 6.5, 15.4))
    cargo = box((-17.7, -5.75, 8), (-9, 5.75, 15.4))
    boat = op(BRepAlgoAPI_Fuse, boat, house, roof, console, sill, stack, cargo, rod)
    return op(BRepAlgoAPI_Cut, boat, box((-2.5, -10, 8.5), (6.8, 10, 27.5)),  # the doors
              cyl((2.15, -10, 27.5), across, 4.65, 20), box((10, -5.3, 24.4), (14, 5.3, 34)),
              cyl((-9, 0, 29.5), along, 4.6, 5), box((-16.2, -4.1, 6.4), (-10.5, 4.1, 16)))  # hold
