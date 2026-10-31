"""A square-to-octagon transition: corners lofted to sides. A transition piece, as a sheet-metal
hood is made, from a square at its foot to an octagon at its top: a loft between two sections with
different numbers of corners.

The square is −20 ≤ x ≤ 20, −20 ≤ y ≤ 20 in the plane z = 0. The octagon lies in the plane
z = 30, centred on the z axis: it is the square 28 × 28 with its corners cut off, its eight
corners (6, −14), (14, −6), (14, 6), (6, 14), (−6, 14), (−14, 6), (−14, −6) and (−6, −14). Its
four sides facing ±x and ±y are 12 long, each parallel to a side of the square below it; its four
cut sides are 8√2 long, each facing a corner of the square.

Each side of the square is joined to the side of the octagon parallel to it by a flat trapezoid:
the square's side from (−20, −20, 0) to (20, −20, 0) to the octagon's from (−6, −14, 30) to
(6, −14, 30), and so on round. Each corner of the square is joined to the cut side of the octagon
facing it by a flat triangle: the corner (20, −20, 0) to the side from (6, −14, 30) to
(14, −6, 30), and so on round. So the loft pairs each of the square's four corners with a whole
side of the octagon, and its eight walls are flat: four trapezoids and four triangles, alternating
round. The solid is the convex hull of the square and the octagon; its floor and top are flat."""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon,
                                BRepBuilderAPI_MakeSolid, BRepBuilderAPI_Sewing)
from OCP.BRepLib import BRepLib
from OCP.TopAbs import TopAbs_SHELL
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Pnt

from bench import Refused

HEIGHT = 30
# counter-clockwise from the corner at (20, −20); each corner of the square is followed by the
# octagon's side facing it, then the next side of the octagon, parallel to the square's next side
SQUARE = [(20, -20), (20, 20), (-20, 20), (-20, -20)]
OCTAGON = [(6, -14), (14, -6), (14, 6), (6, 14), (-6, 14), (-14, 6), (-14, -6), (-6, -14)]


def flat(corners):
    """The flat face bounded by the polygon through `corners`."""
    poly = BRepBuilderAPI_MakePolygon()
    for corner in corners:
        poly.Add(gp_Pnt(*corner))
    poly.Close()
    return BRepBuilderAPI_MakeFace(poly.Wire(), True).Face()


def solid_of(faces):
    """The solid bounded by `faces`, sewn edge to edge into one closed shell."""
    sewing = BRepBuilderAPI_Sewing(1e-6)
    for face in faces:
        sewing.Add(face)
    sewing.Perform()
    shells = TopExp_Explorer(sewing.SewedShape(), TopAbs_SHELL)
    if not shells.More():
        raise Refused("BRepBuilderAPI_Sewing made no shell of the faces")
    make = BRepBuilderAPI_MakeSolid(TopoDS.Shell(shells.Current()))
    if not make.IsDone():
        raise Refused("BRepBuilderAPI_MakeSolid made no solid of the sewn shell")
    solid = make.Solid()
    BRepLib.OrientClosedSolid_s(solid)
    return solid


def build():
    # OCCT's loft (BRepOffsetAPI_ThruSections) joins sections edge to edge, so it cannot pair a
    # corner of the square with a side of the octagon: the hull is sewn from its ten flat faces.
    at = lambda xy, z: (xy[0], xy[1], z)
    faces = [flat([at(p, 0) for p in reversed(SQUARE)]), flat([at(p, HEIGHT) for p in OCTAGON])]
    for k, corner in enumerate(SQUARE):
        before, after = OCTAGON[2 * k - 1], OCTAGON[2 * k]  # the side parallel to the square's
        side_end = OCTAGON[2 * k + 1]                        # the cut side facing the corner
        prev = SQUARE[k - 1]
        faces.append(flat([at(prev, 0), at(corner, 0), at(after, HEIGHT), at(before, HEIGHT)]))
        faces.append(flat([at(corner, 0), at(side_end, HEIGHT), at(after, HEIGHT)]))
    return solid_of(faces)
