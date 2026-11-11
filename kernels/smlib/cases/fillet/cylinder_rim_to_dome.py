"""A cylinder whose rim is rounded into a dome. A cylinder of radius 5 on the z axis, from z = 0
to z = 10. Round the circular edge around its top face to radius 5, the cylinder's own radius, so
that the top face shrinks to nothing and the round is a hemisphere."""
import usd_brep as sm

CYLINDER_RADIUS, HEIGHT = 5, 10
RADIUS = 5
TOLERANCE = 1e-7


def build():
    cylinder = sm.create_cylinder((0, 0, 0), CYLINDER_RADIUS, HEIGHT)
    # The cylinder has three edges: its two rims and the seam of its side. The top rim is the
    # circle at z = 10.
    rim = [edge for edge in cylinder.edges()
           if isinstance(edge.curve(), sm.Circle)
           and abs(edge.curve().center()[2] - HEIGHT) < TOLERANCE]
    assert len(rim) == 1, f"{len(rim)} circles at z = {HEIGHT}"
    return sm.fillet_edges(cylinder, rim, radius=RADIUS)
