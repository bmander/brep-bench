"""A pyramid on a rectangle with every edge rounded, its apex split. A pyramid on the rectangle
from (−10, −6, 0) to (10, 6, 0), its apex at (0, 0, 12). Round all eight of its edges to radius
3/2. The pyramid has no inscribed sphere, so no ball touches all four faces at the apex, and the
apex splits: a short round between the two long sides, closed by a patch of a sphere at each
end."""
import usd_brep as sm

LENGTH, WIDTH, HEIGHT = 20, 12, 12
RADIUS = 1.5


def build():
    # The kernel's pyramid stands on a square: one on the square of the rectangle's length,
    # narrowed along y to the rectangle's width.
    pyramid = sm.create_pyramid((0, 0, 0), LENGTH, HEIGHT)
    sm.scale(pyramid, (1, WIDTH / LENGTH, 1))
    assert pyramid.edge_count() == 8, f"{pyramid.edge_count()} edges"
    return sm.circular_fillet(pyramid, RADIUS)   # rounds every edge, in place
