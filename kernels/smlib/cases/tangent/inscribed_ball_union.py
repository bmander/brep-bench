"""An inscribed ball, united with its cube. A cube of side 10 between the origin and
(10, 10, 10), united with the ball of radius 5 about its centre, which touches each face at one
point: the cube itself."""
import usd_brep as sm

SIDE = 10


def build():
    cube = sm.create_box((0, 0, 0), SIDE, SIDE, SIDE)
    ball = sm.create_sphere((SIDE / 2, SIDE / 2, SIDE / 2), SIDE / 2)
    return sm.boolean_union(cube, ball)
