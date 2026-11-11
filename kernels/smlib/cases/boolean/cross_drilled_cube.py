"""A cube drilled through on a grid three ways. A cube from the origin to (30, 30, 30), drilled by
fifteen holes of radius 2.5: three along x at y = 7.5, 15, 22.5 and z = 15, three along y at
x = 7.5, 15, 22.5 and z = 15, and nine along z through (x, y) for x and y each 7.5, 15, 22.5."""
import usd_brep as sm

SIDE = 30
RADIUS = 2.5
GRID = (7.5, 15, 22.5)
MIDDLE = 15


def drill(through, turn=None):
    """A drill whose axis passes through `through`, running a unit past the cube at both ends.
    It is made along z, and `turn` (an axis and an angle in degrees) lays it along x or y."""
    x, y, z = through
    tool = sm.create_cylinder((x, y, z - MIDDLE - 1), RADIUS, SIDE + 2)
    if turn:
        sm.rotate(tool, through, *turn)
    return tool


def build():
    cube = sm.create_box((0, 0, 0), SIDE, SIDE, SIDE)
    along_x = ((0, 1, 0), 90)   # about +y a quarter turn takes +z to +x
    along_y = ((1, 0, 0), -90)  # about +x a quarter turn back takes +z to +y
    drills = ([drill((MIDDLE, g, MIDDLE), along_x) for g in GRID]
              + [drill((g, MIDDLE, MIDDLE), along_y) for g in GRID]
              + [drill((x, y, MIDDLE)) for x in GRID for y in GRID])
    for tool in drills:
        cube = sm.boolean_difference(cube, tool)
    return cube
