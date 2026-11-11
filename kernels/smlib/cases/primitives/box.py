"""A box: a rectangular block 10 long in x, 20 in y and 30 in z, with one corner at the origin and
the opposite corner at (10, 20, 30)."""
import usd_brep as sm

CORNER = (0, 0, 0)
LENGTH, WIDTH, HEIGHT = 10, 20, 30


def build():
    return sm.create_box(CORNER, LENGTH, WIDTH, HEIGHT)
