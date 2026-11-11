"""Two half balls of one ball, overlapping. The halves z ≥ 0 and x ≥ 0 of the ball of radius 5
about the origin, united: the first's poles on the z axis and the second's on the x axis."""
import usd_brep as sm

RADIUS = 5
REACH = RADIUS + 1   # how far the block that halves the ball reaches, past it


def upper_half():
    """The half z ≥ 0 of the ball, its poles on the z axis as SMLib's sphere has them. SMLib has
    no half-space solid, and its plane cut leaves an open shell: the half is what the ball has in
    common with a block standing on the plane z = 0."""
    ball = sm.create_sphere((0, 0, 0), RADIUS)
    above = sm.create_box((-REACH, -REACH, 0), 2 * REACH, 2 * REACH, REACH)
    return sm.boolean_intersection(ball, above)


def build():
    upper = upper_half()
    # the same half turned from the z axis to the x axis: the half x ≥ 0, its poles on the x axis
    front = sm.rotate(upper_half(), (0, 0, 0), (0, 1, 0), 90)
    return sm.boolean_union(upper, front)
