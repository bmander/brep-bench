"""A tangent tee, united. A main cylinder of radius 5 along the x axis from x = −15 to x = 15,
united with a branch of radius 3 parallel to y from y = −15 to y = 15 through x = 0, z = 2. The
branch's wall touches the main's from inside at (0, 0, 5)."""
import usd_brep as sm

MAIN_RADIUS = 5
BRANCH_RADIUS = 3
BRANCH_Z = 2
HALF_LENGTH = 15


def build():
    main = sm.create_cylinder((0, 0, -HALF_LENGTH), MAIN_RADIUS, 2 * HALF_LENGTH)
    sm.rotate(main, (0, 0, 0), (0, 1, 0), 90)        # its axis from z to x
    branch = sm.create_cylinder((0, 0, -HALF_LENGTH), BRANCH_RADIUS, 2 * HALF_LENGTH)
    sm.rotate(branch, (0, 0, 0), (1, 0, 0), -90)     # its axis from z to y
    sm.translate(branch, (0, 0, BRANCH_Z))
    return sm.boolean_union(main, branch)
