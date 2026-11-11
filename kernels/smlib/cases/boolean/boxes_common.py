"""Two overlapping blocks, their common part. Two blocks 10 high, both standing on the xy plane.
The first spans x and y from 0 to 10; the second spans x and y from 5 to 15. Make the part
common to both."""
import usd_brep as sm

HEIGHT = 10


def build():
    # A box is given by its lowest corner and its lengths along x, y and z.
    first = sm.create_box((0, 0, 0), 10, 10, HEIGHT)
    second = sm.create_box((5, 5, 0), 10, 10, HEIGHT)
    return sm.boolean_intersection(first, second)
