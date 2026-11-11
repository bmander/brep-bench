"""A torus about the z axis, centred on the origin, of radii 5 (its middle circle) and 3 (its
tube), cut by the plane 3x − 4z = 0, keeping every point with z ≤ 3x/4. The plane is tangent to
the torus at two points and cuts it in Villarceau's circles."""
import usd_brep as sm

MAJOR_RADIUS = 5
MINOR_RADIUS = 3
REACH = MAJOR_RADIUS + MINOR_RADIUS + 2   # how far the cutting block reaches, past the torus


def build():
    torus = sm.create_torus((0, 0, 0), MAJOR_RADIUS, MINOR_RADIUS)
    # SMLib's torus has its seam meridian on the +x side: a quarter turn puts it in the plane
    # x = 0, on the side y > 0, away from the tangent points
    sm.rotate(torus, (0, 0, 0), (0, 0, 1), 90)
    # What is cut away is a block standing on the plane: its floor's x axis is placed along
    # (4, 0, 3) and its y along the world's, so its height is along the plane's normal (−3, 0, 4).
    above = sm.create_box((-REACH, -REACH, 0), 2 * REACH, 2 * REACH, REACH)
    sm.transform(above, sm.Axis2Placement((0, 0, 0), (0.8, 0, 0.6), (0, 1, 0)))
    return sm.boolean_difference(torus, above)
