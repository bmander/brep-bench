"""A tugboat test print (a benchy): 60 long, 30 wide and 48 high, symmetric about y = 0, its stern
at x = 0, its bow toward +x and the flat of its keel on z = 0. The union of six parts less the
openings cut into them:

1. the hull, the common part of the plan P extruded along z, the profile S extruded along y and
   the chined section z ≥ |y| − 10 extruded along x. P: the transom x = 0 (|y| ≤ 10), quarter
   circles of radius 5 about (5, ±10), the sides y = ±15 to x = 30, and arcs of radius 37.5 about
   (30, ∓22.5) meeting at the stem (60, 0). S: below the sheer z = 14 + x/10 and above the keel
   line, the counter (radius 20 about (x, z) = (12, 20), from (0, 4) to (12, 0)), the flat keel
   z = 0 to x = 40, and the stem (radius 25 about (40, 25), from (40, 0) to (60, 10));
2. the deck well taken out of it: z ≥ 9 over the plan moved in by 1.5;
3. the wheelhouse: walls between the rounded rectangles [12, 32] × [−10, 10] (radius 2) and
   [13.5, 30.5] × [−8.5, 8.5] (radius 0.5), from the deck up to the roof's underside
   z = 30 + (x − 12)/10; the roof, a slab 1.5 thick above that plane over [10, 34] × [−12, 12]
   (radius 3); arched doors (16 ≤ x ≤ 22, 9 ≤ z ≤ 20 and a half disk of radius 3 about (19, 20))
   through both side walls; a front window (|y| ≤ 7, 22 ≤ z ≤ 28, radius 1.5) and a round rear
   window (radius 4 about (y, z) = (0, 22));
4. the chimney, radius 4 about the vertical through (18, 0), from z = 31.5 (inside the roof) to
   48, bored to radius 2.75 down to z = 38;
5. the cargo box, [2.5, 9.5] × [−5, 5] × [9, 14] less [3.5, 8.5] × [−4, 4] × [10, 14];
6. the rod holder, a tube of radius 1.5 about the line through (8, 9, 9) along (−1/2, 0, √3/2),
   between the planes z = 8 and z = 15, bored to radius 0.9 down to z = 10;
7. and the hawse holes, radius 1.25 about the line along y through (52, 0, 16).

Each region of a plane is a face made from its outline, and extruded (BRepPrimAPI_MakePrism) past
where it is cut, so that no two faces of a Boolean's operands lie on each other but where the boat
itself has parts meeting face to face (the deck, the roof's underside)."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakePrism
from OCP.GC import GC_MakeArcOfCircle, GC_MakeSegment
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt, gp_Vec

from bench import Refused

SQ3 = math.sqrt(3)


def outline(plane, pieces):
    """The face inside a closed outline in a coordinate plane. `plane` maps the plane's two
    coordinates to a point in space; each piece is ("line", a, b) or ("arc", center, a, b), the
    arc running from a to b the short way about its center."""
    wire = BRepBuilderAPI_MakeWire()
    for piece in pieces:
        if piece[0] == "line":
            curve = GC_MakeSegment(gp_Pnt(*plane(*piece[1])), gp_Pnt(*plane(*piece[2]))).Value()
        else:
            _, c, a, b = piece
            r = math.dist(c, a)
            # the arc's middle: the bisector of a and b from the center, at the radius
            m = ((a[0] + b[0]) / 2 - c[0], (a[1] + b[1]) / 2 - c[1])
            k = r / math.hypot(*m)
            mid = (c[0] + k * m[0], c[1] + k * m[1])
            curve = GC_MakeArcOfCircle(gp_Pnt(*plane(*a)), gp_Pnt(*plane(*mid)),
                                       gp_Pnt(*plane(*b))).Value()
        wire.Add(BRepBuilderAPI_MakeEdge(curve).Edge())
    face = BRepBuilderAPI_MakeFace(wire.Wire(), True)
    if not face.IsDone():
        raise Refused("BRepBuilderAPI_MakeFace declined an outline")
    return face.Face()


def xy(z):
    return lambda x, y: (x, y, z)


def xz(y):
    return lambda x, z: (x, y, z)


def yz(x):
    return lambda y, z: (x, y, z)


def prism(face, v):
    return BRepPrimAPI_MakePrism(face, gp_Vec(*v)).Shape()


def rounded_rectangle(x0, x1, y0, y1, r):
    """The outline of [x0, x1] × [y0, y1] with its corners rounded to r, in a plane's two
    coordinates."""
    return [("line", (x0 + r, y0), (x1 - r, y0)), ("arc", (x1 - r, y0 + r), (x1 - r, y0), (x1, y0 + r)),
            ("line", (x1, y0 + r), (x1, y1 - r)), ("arc", (x1 - r, y1 - r), (x1, y1 - r), (x1 - r, y1)),
            ("line", (x1 - r, y1), (x0 + r, y1)), ("arc", (x0 + r, y1 - r), (x0 + r, y1), (x0, y1 - r)),
            ("line", (x0, y1 - r), (x0, y0 + r)), ("arc", (x0 + r, y0 + r), (x0, y0 + r), (x0 + r, y0))]


def plan(inset):
    """The hull's plan moved in by `inset`: the transom, the stern's quarter circles, the sides and
    the bow's two arcs, which meet on y = 0."""
    r, side, bow = 5 - inset, 15 - inset, 37.5 - inset
    tip = 30 + math.sqrt(bow ** 2 - 22.5 ** 2)
    return [("line", (inset, -10), (inset, 10)), ("arc", (5, 10), (inset, 10), (5, side)),
            ("line", (5, side), (30, side)), ("arc", (30, -22.5), (30, side), (tip, 0)),
            ("arc", (30, 22.5), (tip, 0), (30, -side)), ("line", (30, -side), (5, -side)),
            ("arc", (5, -10), (5, -side), (inset, -10))]


def between(z0, z1, x0=-5, x1=65, slope=0.1):
    """Everything between the planes z = z0 + slope·(x − 12) and z = z1 + slope·(x − 12), for
    x0 ≤ x ≤ x1, across all of y: the slab the walls and the roof are cut to."""
    at = lambda x, z: (x, z + slope * (x - 12))
    face = outline(xz(-20), [("line", at(x0, z0), at(x1, z0)), ("line", at(x1, z0), at(x1, z1)),
                             ("line", at(x1, z1), at(x0, z1)), ("line", at(x0, z1), at(x0, z0))])
    return prism(face, (0, 40, 0))


def common(a, b, what):
    op = BRepAlgoAPI_Common(a, b)
    if not op.IsDone():
        raise Refused(f"BRepAlgoAPI_Common declined {what}")
    return op.Shape()


def fuse(a, b, what):
    op = BRepAlgoAPI_Fuse(a, b)
    if not op.IsDone():
        raise Refused(f"BRepAlgoAPI_Fuse declined {what}")
    return op.Shape()


def cut(a, b, what):
    op = BRepAlgoAPI_Cut(a, b)
    if not op.IsDone():
        raise Refused(f"BRepAlgoAPI_Cut declined {what}")
    return op.Shape()


def hull():
    above_plan = prism(outline(xy(-2), plan(0)), (0, 0, 30))
    # the profile, run past x = 0 and x = 60 along its circles: the counter from x = −1, the stem
    # to x = 62, then up past the sheer and back
    counter = (-1, 20 - math.sqrt(400 - 13 ** 2))
    stem = (62, 25 - math.sqrt(625 - 22 ** 2))
    profile = outline(xz(-20), [
        ("arc", (12, 20), counter, (12, 0)), ("line", (12, 0), (40, 0)),
        ("arc", (40, 25), (40, 0), stem), ("line", stem, (62, 14 + 6.2)),
        ("line", (62, 20.2), (-1, 13.9)), ("line", (-1, 13.9), counter)])
    along_profile = prism(profile, (0, 40, 0))
    # the chines: z ≥ |y| − 10, a V whose point is under the keel, closed high above the sheer
    chines = outline(yz(-2), [("line", (0, -10), (30, 20)), ("line", (30, 20), (30, 30)),
                              ("line", (30, 30), (-30, 30)), ("line", (-30, 30), (-30, 20)),
                              ("line", (-30, 20), (0, -10))])
    along_section = prism(chines, (66, 0, 0))
    h = common(common(above_plan, along_profile, "the plan and the profile"), along_section,
               "the chines")
    well = prism(outline(xy(9), plan(1.5)), (0, 0, 20))
    return cut(h, well, "the deck well")


def wheelhouse():
    outer = prism(outline(xy(9), rounded_rectangle(12, 32, -10, 10, 2)), (0, 0, 30))
    inner = prism(outline(xy(8), rounded_rectangle(13.5, 30.5, -8.5, 8.5, 0.5)), (0, 0, 31))
    walls = common(cut(outer, inner, "the room"), between(-30, 30), "the walls' tops")
    roof = common(prism(outline(xy(25), rounded_rectangle(10, 34, -12, 12, 3)), (0, 0, 15)),
                  between(30, 31.5), "the roof")
    house = fuse(walls, roof, "the roof to the walls")
    door = outline(xz(-11), [("line", (16, 9), (22, 9)), ("line", (22, 9), (22, 20)),
                             ("arc", (19, 20), (22, 20), (19, 23)),
                             ("arc", (19, 20), (19, 23), (16, 20)), ("line", (16, 20), (16, 9))])
    house = cut(house, prism(door, (0, 22, 0)), "the doors")
    window = outline(yz(30), rounded_rectangle(-7, 7, 22, 28, 1.5))
    house = cut(house, prism(window, (3, 0, 0)), "the front window")
    rear = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(11, 0, 22), gp_Dir(1, 0, 0)), 4, 3).Shape()
    return cut(house, rear, "the rear window")


def chimney():
    up = gp_Dir(0, 0, 1)
    stack = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(18, 0, 31.5), up), 4, 16.5).Shape()
    bore = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(18, 0, 38), up), 2.75, 11).Shape()
    return cut(stack, bore, "the chimney's bore")


def cargo_box():
    box = BRepPrimAPI_MakeBox(gp_Pnt(2.5, -5, 9), gp_Pnt(9.5, 5, 14)).Shape()
    hold = BRepPrimAPI_MakeBox(gp_Pnt(3.5, -4, 10), gp_Pnt(8.5, 4, 15)).Shape()
    return cut(box, hold, "the cargo box's hold")


def rod_holder():
    lean = gp_Dir(-0.5, 0, SQ3 / 2)
    # the axis from where it is at z = 6, well under the deck, for 12 along it (to z ≈ 16.4)
    start = gp_Pnt(8 + 3 / SQ3, 9, 6)
    tube = BRepPrimAPI_MakeCylinder(gp_Ax2(start, lean), 1.5, 12).Shape()
    bore = BRepPrimAPI_MakeCylinder(gp_Ax2(start, lean), 0.9, 12).Shape()
    level = lambda z0, z1: BRepPrimAPI_MakeBox(gp_Pnt(0, 5, z0), gp_Pnt(14, 13, z1)).Shape()
    tube = common(tube, level(8, 15), "the rod holder's ends")
    return cut(tube, common(bore, level(10, 20), "the rod holder's bore"), "the rod holder's bore")


def build():
    boat = hull()
    for part, what in ((wheelhouse(), "the wheelhouse"), (chimney(), "the chimney"),
                       (cargo_box(), "the cargo box"), (rod_holder(), "the rod holder")):
        boat = fuse(boat, part, f"{what} to the boat")
    hawse = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(52, -12, 16), gp_Dir(0, 1, 0)), 1.25, 24)
    return cut(boat, hawse.Shape(), "the hawse holes")
