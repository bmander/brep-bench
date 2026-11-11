"""A cylinder whose rim is chamfered to a point. A cylinder of radius 5 on the z axis, from z = 0
to z = 10. Chamfer the circular edge around its top face with the same distance 5 on both faces,
the cylinder's own radius: the top face shrinks to its centre, and the chamfer is the cone from
the circle of radius 5 at z = 5 up to its apex at (0, 0, 10)."""
import usd_brep as sm

CYLINDER_RADIUS, HEIGHT = 5, 10
DISTANCE = 5
LINEAR = 0   # fillet_edges' flat cross-section: the round's two tangent lines joined by a plane
TOLERANCE = 1e-7


def build():
    cylinder = sm.create_cylinder((0, 0, 0), CYLINDER_RADIUS, HEIGHT)
    rim = [edge for edge in cylinder.edges()
           if isinstance(edge.curve(), sm.Circle)
           and abs(edge.curve().center()[2] - HEIGHT) < TOLERANCE]
    assert len(rim) == 1, f"{len(rim)} circles at z = {HEIGHT}"
    # A flat cut between the lines where a round of that radius would touch the faces: at a
    # square edge, the lines at the radius from it.
    return sm.fillet_edges(cylinder, rim, radius=DISTANCE, xsect_type=LINEAR)
