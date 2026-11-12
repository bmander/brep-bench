"""A dome machined in five axes by four cutters in 238 passes. The blank is the solid cylinder of
radius 50 about the z axis from z = 0 to 45; every move is a slide, a turn about the z axis, or a
turn about a level line through the origin, given as the cutter's motion relative to the part.

1. A bull-nose mill of radius 8, its corner rounded to 2, 40 long, upright, roughs eight levels,
   its tip at z = 41, 36, … 6: at each, circles about the z axis of radius √(40² − z²) + 9 + 12m
   short of 58, the largest first, each plunged from z = 50 and risen back.
2. A ball-end mill of radius 4, 40 long, held square to the dome (the ball of radius 40 about the
   origin), its tip at 40·n(λ, φ) on the normal n = (cos λ cos φ, cos λ sin φ, sin λ): 180 passes up
   the meridians φ = 0°, 2°, … 358°, each slid in from 90·n(3°, φ), turned about the level line
   along (sin φ, −cos φ, 0) by 72° to latitude 75°, slid out, turned back and round the z axis by
   2°; 15 passes round the pole at λ = 75°, … 89°, each slid in, turned once round the z axis, slid
   out; and a touch at the pole, from (0, 0, 90) to (0, 0, 40) and back.
3. A flat end mill of radius 5, 40 long, faces a seat at latitude 40° and φ = 15°, 45°, … 345°,
   square to the dome, its tip slid from 70·n to 38.5·n and back; and
4. a drill of radius 2.5 with a 90° point, 40 long, drills each, from 70·n to 30·n and back.

The solid is the blank less everything the four cutters pass through."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp

from cnc import Program, ball, bull, cut_all, drill, flat

DOME = 40
CLEAR, NEAR = 90, 70


def normal(lat, lon):
    l, p = math.radians(lat), math.radians(lon)
    return (math.cos(l) * math.cos(p), math.cos(l) * math.sin(p), math.sin(l))


def scaled(v, k):
    return tuple(k * x for x in v)


def roughing():
    rings = []
    for z in (41, 36, 31, 26, 21, 16, 11, 6):
        rho = math.sqrt(max(0, DOME ** 2 - z ** 2))
        radii = []
        r = rho + 9
        while r < 58:
            radii.append(r)
            r += 12
        rings += [(r, z) for r in reversed(radii)]
    p = Program(bull(8, 2, 40), (rings[0][0], 0, 50))
    for r, z in rings:
        p.line((r, 0, 50))
        p.line((r, 0, z))
        p.arc((0, 0), (r, 0, z))
        p.line((r, 0, 50))
    return p


def along_axis(p, by):
    tip, a = p.tip(), p.axis()
    return p.line((tip[0] + by * a.X(), tip[1] + by * a.Y(), tip[2] + by * a.Z()))


def finishing():
    p = Program(ball(4, 40), scaled(normal(3, 0), CLEAR), normal(3, 0))
    reach = CLEAR - DOME
    for k in range(180):
        lon = math.radians(2 * k)
        line = (math.sin(lon), -math.cos(lon), 0)
        along_axis(p, -reach)
        p.turn((0, 0, 0), line, 72)
        along_axis(p, reach)
        p.turn((0, 0, 0), line, -72)
        p.turn((0, 0, 0), (0, 0, 1), 2)
    p.turn((0, 0, 0), (0, -1, 0), 72)
    for _ in range(15):
        along_axis(p, -reach)
        p.turn((0, 0, 0), (0, 0, 1), 360)
        along_axis(p, reach)
        p.turn((0, 0, 0), (0, -1, 0), 1)
    along_axis(p, -reach)
    along_axis(p, reach)
    return p


def square_to_the_dome(cutter, depth):
    p = Program(cutter, scaled(normal(40, 15), NEAR), normal(40, 15))
    for k in range(12):
        along_axis(p, depth - NEAR)
        along_axis(p, NEAR - depth)
        if k < 11:
            p.turn((0, 0, 0), (0, 0, 1), 30)
    return p


def build():
    blank = BRepPrimAPI_MakeCylinder(gp.XOY_s(), 50, 45).Shape()
    swept = []
    for p in (roughing(), finishing(), square_to_the_dome(flat(5, 40), 38.5),
              square_to_the_dome(drill(2.5, 45, 40), 30)):
        swept += p.swept
    return cut_all(blank, swept)
