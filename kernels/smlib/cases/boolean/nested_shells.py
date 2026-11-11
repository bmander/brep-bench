"""Three nested bodies that do not touch. Five balls: of radius 10 at the origin, 8 at (1, 0, 0),
6 at (1, 1, 0), 4 at (1, 1, 1) and 2 at (1, 1.5, 1.5), each inside the one before. The solid is
the first less the second, joined with the third less the fourth, joined with the fifth: a hollow
ball holds a hollow ball, which holds a ball."""
import usd_brep as sm


def build():
    outer = sm.boolean_difference(sm.create_sphere((0, 0, 0), 10), sm.create_sphere((1, 0, 0), 8))
    middle = sm.boolean_difference(sm.create_sphere((1, 1, 0), 6), sm.create_sphere((1, 1, 1), 4))
    both = sm.boolean_union(outer, middle)
    return sm.boolean_union(both, sm.create_sphere((1, 1.5, 1.5), 2))
