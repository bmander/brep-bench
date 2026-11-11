"""A cube on a cube, overhanging. Two cubes of side 10, joined into one solid. The first lies
between the origin and (10, 10, 10). The second lies between (5, 0, 10) and (15, 10, 20): it
stands on the first, and its bottom covers half of the first's top."""
import usd_brep as sm

SIDE = 10


def build():
    lower = sm.create_box((0, 0, 0), SIDE, SIDE, SIDE)
    upper = sm.create_box((5, 0, 10), SIDE, SIDE, SIDE)
    return sm.boolean_union(lower, upper)
