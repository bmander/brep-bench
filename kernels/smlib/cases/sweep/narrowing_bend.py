"""A hollow square duct narrowing around a bend. Its centreline is an arc of radius 30 in the xy
plane about the z axis, from (30, 0, 0) to (0, 30, 0). At every point of the arc the section lies
in the vertical plane through the z axis: a square centred on the arc, two sides horizontal, with
a square hole at its centre leaving a wall 2 thick. The section's outer half-width h runs
linearly with the angle turned, from 9 at the start to 6 at the end, and the hole's is h − 2.
Each of the eight long faces is swept by one straight side of the section, its ends moving along
the spirals the section's corners trace. The two ends are flat."""
import usd_brep as sm

BEND_RADIUS = 30
TURN = 90   # degrees
HALF_START, HALF_END = 9, 6   # the outside's half-width
WALL = 2
HELIX_TOLERANCE = 1e-6   # a helix is a B-spline fitted to within this, the tightest it takes

# The section's corners, in order around it: outward (1) or inward (−1) of the arc, below it
# (False) or above (True).
CORNERS = [(1, False), (1, True), (-1, True), (-1, False)]


def spiral(start, end, outward, above):
    """The curve a corner of the square traces as its half-width runs from `start` to `end`: a
    conical helix about the z axis, its radius and its height both linear in the angle."""
    # A helix starts on the +x side and climbs: this is a lower corner's, from `start` below the
    # arc to `end` below it. An upper corner's is its mirror image in the arc's plane.
    helix = sm.create_helix((0, 0, -start), start - end, BEND_RADIUS + outward * start,
                            BEND_RADIUS + outward * end, TURN / 360, tolerance=HELIX_TOLERANCE)
    return sm.scale(helix, (1, 1, -1)) if above else helix


def long_faces(start, end):
    """The four long faces of the square of half-width `start` narrowing to `end` around the
    bend: a skin of degree 1 around its four corners' spirals and back to the first, left open
    at its ends. Each face is ruled by the straight lines joining two spirals at the same
    angle."""
    spirals = [spiral(start, end, *corner) for corner in CORNERS + CORNERS[:1]]
    return sm.create_skin_primitive([1] * len(spirals), spirals, degree=1, cap_ends=False)


def end_face(half, hole, at_end):
    """The flat end: the square of half-width `half` with the hole of half-width `hole`, in the
    xz plane at x = 30 or, `at_end`, in the yz plane at y = 30."""
    def square(h):
        corners = [(BEND_RADIUS + outward * h, h if above else -h) for outward, above in CORNERS]
        corners = [(0, rho, z) if at_end else (rho, 0, z) for rho, z in corners]
        return [sm.create_line_segment(a, b) for a, b in zip(corners, corners[1:] + corners[:1])]
    return sm.create_planar_faces(square(half) + square(hole))


def build():
    # Workaround: SMLib's sweep with a scale (curve_sweep) scales the section by the distance
    # between two curves at the same parameter, and an arc's parameter does not keep step with
    # its angle, so the section's width cannot be made linear in the angle turned. The same
    # solid is closed from its ten faces: the outside's four long faces and the hole's, and the
    # two flat ends.
    faces = [long_faces(HALF_START, HALF_END), long_faces(HALF_START - WALL, HALF_END - WALL),
             end_face(HALF_START, HALF_START - WALL, False),
             end_face(HALF_END, HALF_END - WALL, True)]
    # Merged, the faces share their edges and close a shell; healing it classifies what the
    # shell encloses as solid.
    duct = sm.merge_breps(faces, operation=sm.BooleanOp.MERGE)
    sm.heal_brep(duct)
    assert duct.is_manifold_solid(), "the ten faces did not close into a solid"
    return duct
