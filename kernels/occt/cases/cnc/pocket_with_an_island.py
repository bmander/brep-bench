"""A 2.5-axis program of three tools. The plate is the box from (0, 0, −20) to (160, 110, 0). A flat
end mill of radius 5 clears a pocket — the rectangle (20, 20)–(140, 90) rounded to 12, less the
disc of radius 15 about (60, 55) — at z = −4, −8 and −12: raster passes along y = 26, 30, … 82
across the region its tip keeps to (inside the wall path, the rectangle (25, 25)–(135, 85) rounded
to 7, outside the island path, the circle of radius 20 about (60, 55)), then once round each path.
A flat end mill of radius 3 bores the island's top to radius 9, 6 deep; a drill of radius 4 with a
90° point pecks four holes, its tip to z = −14. Every move between cuts is at z = 5. The solid is
the plate less everything the three cutters pass through."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from cnc import Program, cut_all, drill, flat, rounded_loop

CLEAR = 5
WALL = [(25, 25), (135, 25), (135, 85), (25, 85)]
WALL_ROUND = 7
ISLAND = (60, 55)
ISLAND_PATH = 20
HOLES = [(8, 8), (152, 8), (152, 102), (8, 102)]


def across_wall(y):
    lo, hi = WALL[0][1] + WALL_ROUND, WALL[2][1] - WALL_ROUND
    inset = lo - y if y < lo else (y - hi if y > hi else 0)
    x = WALL_ROUND - math.sqrt(WALL_ROUND ** 2 - inset ** 2)
    return WALL[0][0] + x, WALL[1][0] - x


def raster(y):
    a, b = across_wall(y)
    dy = y - ISLAND[1]
    if abs(dy) >= ISLAND_PATH:
        return [(a, b)]
    half = math.sqrt(ISLAND_PATH ** 2 - dy ** 2)
    return [(a, ISLAND[0] - half), (ISLAND[0] + half, b)]


def plunged(p, at, depth, cut):
    p.line((at[0], at[1], CLEAR))
    p.line((at[0], at[1], depth))
    cut(p)
    end = p.tip()
    p.line((end[0], end[1], CLEAR))


def build():
    plate = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, -20), gp_Pnt(160, 110, 0)).Shape()
    pocket = Program(flat(5, 30), (ISLAND[0], ISLAND[1], CLEAR))
    for z in (-4, -8, -12):
        for k in range(15):
            y = 26 + 4 * k
            for a, b in raster(y):
                plunged(pocket, (a, y), z, lambda p, b=b, y=y, z=z: p.line((b, y, z)))
        plunged(pocket, (80, 25), z, lambda p, z=z: rounded_loop(p, WALL, WALL_ROUND, z))
        point = (ISLAND[0] + ISLAND_PATH, ISLAND[1])
        plunged(pocket, point, z,
                lambda p, z=z: p.arc(ISLAND, (point[0], point[1], z), anticlockwise=False))
    bore = Program(flat(3, 30), (ISLAND[0], ISLAND[1], CLEAR))
    for z in (-3, -6):
        bore.line((ISLAND[0], ISLAND[1], z))
        bore.line((ISLAND[0] + 6, ISLAND[1], z))
        bore.arc(ISLAND, (ISLAND[0] + 6, ISLAND[1], z))
        bore.line((ISLAND[0], ISLAND[1], z))
    bore.line((ISLAND[0], ISLAND[1], CLEAR))
    holes = Program(drill(4, 45, 30), (HOLES[0][0], HOLES[0][1], CLEAR))
    for x, y in HOLES:
        holes.line((x, y, CLEAR))
        for down, up in ((-5, 2), (-10, 2), (-14, CLEAR)):
            holes.line((x, y, down))
            holes.line((x, y, up))
    return cut_all(plate, pocket.swept + bore.swept + holes.swept)
