"""An ice-cream cone. A ball of radius 15 centred on the origin, united with a cone on the z axis
whose apex is at (0, 0, −25) and whose base is the disk of radius 12 at z = −9: the cone's side
is tangent to the sphere along the rim of its base."""
import usd_brep as sm

BALL_RADIUS = 15
APEX_Z = -25
CONE_RADIUS, CONE_HEIGHT = 12, 16


def build():
    ball = sm.create_sphere((0, 0, 0), BALL_RADIUS)
    # the cone's first radius is at its origin: nothing there, the apex
    cone = sm.create_cone((0, 0, APEX_Z), 0, CONE_RADIUS, CONE_HEIGHT)
    return sm.boolean_union(ball, cone)
