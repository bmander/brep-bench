"""A cube with a half-round end. Take a cube of side 10 between the origin and (10, 10, 10).
Unite it with a cylinder of radius 5 and height 10 that stands on the xy plane, its axis at
x = 10, y = 5. Half of the cylinder lies inside the cube. Its surface touches the cube's faces
y = 0 and y = 10 exactly along the cube's edges at x = 10."""
import usd_brep as sm

SIDE = 10
BOSS_RADIUS = 5
BOSS_HEIGHT = 10


def build():
    cube = sm.create_box((0, 0, 0), SIDE, SIDE, SIDE)
    boss = sm.create_cylinder((10, 5, 0), BOSS_RADIUS, BOSS_HEIGHT)
    return sm.boolean_union(cube, boss)
