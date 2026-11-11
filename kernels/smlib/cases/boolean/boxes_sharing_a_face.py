"""Two cubes joined face to face. Two cubes of side 10, side by side: the first between the
origin and (10, 10, 10), the second between (10, 0, 0) and (20, 10, 10). Join them into one
solid. They touch along a whole face, which must disappear from the result."""
import usd_brep as sm

SIDE = 10


def build():
    first = sm.create_box((0, 0, 0), SIDE, SIDE, SIDE)
    second = sm.create_box((10, 0, 0), SIDE, SIDE, SIDE)
    return sm.boolean_union(first, second)
