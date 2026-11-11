"""A NURBS cylinder with its top rim rounded. A cylinder of radius 5 on the z axis, from z = 0 to
z = 10, whose side is one rational B-spline surface: the standard rational quadratic circle of
nine control points, on the knots (0, 0, 0, ¼, ¼, ½, ½, ¾, ¾, 1, 1, 1), times a straight line.
Its ends are flat discs. Round the circular edge where the side meets the top to radius 1."""
import math

import usd_brep as sm

CYLINDER_RADIUS, HEIGHT = 5, 10
RADIUS = 1
TOLERANCE = 1e-7

W = math.sqrt(2) / 2
CIRCLE = [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0)]
CIRCLE_WEIGHTS = [1, W, 1, W, 1, W, 1, W, 1]
CIRCLE_KNOTS, CIRCLE_MULTIPLICITIES = [0, 0.25, 0.5, 0.75, 1], [3, 2, 2, 2, 3]


def circle(z):
    """The rational quadratic circle of radius 5 about the z axis at height z."""
    points = [(CYLINDER_RADIUS * x, CYLINDER_RADIUS * y, z) for x, y in CIRCLE]
    return sm.create_canonical_curve(points, CIRCLE_KNOTS, CIRCLE_MULTIPLICITIES,
                                     weights=CIRCLE_WEIGHTS, degree=2)


def build():
    # The side is the skin of degree 1 between the circle at z = 0 and the circle at z = 10: the
    # circle's net times a straight line, kept as a B-spline surface with its seam at y = 0,
    # x > 0. The skin's two ends are capped flat. (Swept straight up instead, the circle is
    # recognised and the side made a cylinder.)
    cylinder = sm.create_skin_primitive([1, 1], [circle(0), circle(HEIGHT)], degree=1)
    # The top rim is the edge whose ends are at z = 10; the seam's are at different heights.
    rim = [edge for edge in cylinder.edges()
           if all(abs(vertex.point()[2] - HEIGHT) < TOLERANCE for vertex in edge.vertices())]
    assert len(rim) == 1, f"{len(rim)} edges at z = {HEIGHT}"
    return sm.fillet_edges(cylinder, rim, radius=RADIUS)
