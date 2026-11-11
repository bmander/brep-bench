"""A picture frame: a rabbeted moulding mitred around a rectangle. The path: the rectangle in the
xy plane with corners (±30, ±20, 0), closed. The section stands square to the path, given by s,
the distance outward from the path, and z: the polygon through (s, z) = (0, 2), (3, 2), (3, 0),
(8, 0), (8, 6), (0, 6), a bar 8 wide and 6 tall outside the path with a rabbet 3 wide and 2 deep
out of its inner bottom edge. Each side is the section swept straight along it, and at each
corner the two sides are mitred in the vertical plane along the corner's diagonal."""
import usd_brep as sm

HALF_LENGTH, HALF_WIDTH = 30, 20
SECTION = [(0, 2), (3, 2), (3, 0), (8, 0), (8, 6), (0, 6)]   # (s, z)


def polyline(points, closed=False):
    """Straight lines from each point to the next, and back to the first if `closed`."""
    points = points + points[:1] if closed else points
    return [sm.create_line_segment(a, b) for a, b in zip(points, points[1:])]


def build():
    # The section rides the path by its first point, which here is (s, z) = (0, 2): that corner
    # of the section runs along the rectangle 2 above the xy plane, and so the path is drawn
    # there.
    s0, z0 = SECTION[0]
    assert s0 == 0, "the section's first corner is not over the path"
    corners = [(-HALF_LENGTH, -HALF_WIDTH), (HALF_LENGTH, -HALF_WIDTH),
               (HALF_LENGTH, HALF_WIDTH), (-HALF_LENGTH, HALF_WIDTH)]
    path = polyline([(x, y, z0) for x, y in corners], closed=True)
    # The section where the path starts, at (−30, −20), where it sets off along +x and outward
    # is −y. A section keeps the way it is drawn when its outline runs counter-clockwise about
    # the path's direction, which is the given outline backwards from its first point.
    outline = SECTION[:1] + SECTION[:0:-1]
    section = polyline([(-HALF_LENGTH, -HALF_WIDTH - s, z) for s, z in outline], closed=True)
    # A path in one plane is swept with its corners mitred, and a closed one has no ends.
    return sm.sweep_along_planar_path(section, path, move_profile=False)
