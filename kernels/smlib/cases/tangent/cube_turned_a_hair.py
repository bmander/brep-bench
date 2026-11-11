"""A cube united with itself turned a hair. A cube of side 10 from (−5, −5, 0) to (5, 5, 10),
united with a copy of itself turned 0.001° about the z axis."""
import usd_brep as sm

SIDE = 10
TURN = 0.001   # degrees


def build():
    cube = sm.create_box((-SIDE / 2, -SIDE / 2, 0), SIDE, SIDE, SIDE)
    turned = sm.rotate(cube.copy(), (0, 0, 0), (0, 0, 1), TURN)
    return sm.boolean_union(cube, turned)
