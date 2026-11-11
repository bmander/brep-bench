"""A ball of radius 10 centred on the origin, and a solid cylinder of radius 5 whose axis is the
line x = 5, y = 0, parallel to the z axis, from z = −12 to z = 12. Make the solid common to both. The cylinder
touches the sphere from inside at (10, 0, 0), where Viviani's curve crosses itself."""
import usd_brep as sm

RADIUS = 10
CYLINDER_RADIUS = RADIUS / 2
HALF_HEIGHT = 12


def build():
    # Each seam is put where the case asks, away from the curve's crossing at (10, 0, 0). SMLib's
    # sphere has its poles on its frame's z axis and its seam on the frame's +x side: the frame
    # is placed with z along the world's y and x along the world's −x.
    ball = sm.create_sphere((0, 0, 0), RADIUS)
    sm.transform(ball, sm.Axis2Placement((0, 0, 0), (-1, 0, 0), (0, 0, 1)))
    # the cylinder's seam is on its +x side: half a turn about its axis puts it on the z axis
    foot = (CYLINDER_RADIUS, 0, -HALF_HEIGHT)
    cylinder = sm.create_cylinder(foot, CYLINDER_RADIUS, 2 * HALF_HEIGHT)
    sm.rotate(cylinder, foot, (0, 0, 1), 180)
    return sm.boolean_intersection(ball, cylinder)
