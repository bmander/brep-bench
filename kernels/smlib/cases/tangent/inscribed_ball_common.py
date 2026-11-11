"""An inscribed ball, in common with its cube. A cube of side 10 between the origin and
(10, 10, 10), and the ball of radius 5 about its centre, which touches each face at one point:
what is common to both, the whole ball."""
import usd_brep as sm

SIDE = 10


def build():
    cube = sm.create_box((0, 0, 0), SIDE, SIDE, SIDE)
    ball = sm.create_sphere((SIDE / 2, SIDE / 2, SIDE / 2), SIDE / 2)
    return sm.boolean_intersection(cube, ball)
