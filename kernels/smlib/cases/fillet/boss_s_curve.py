"""A boss whose root and rim rounds meet. A square plate from (−20, −20, 0) to (20, 20, 4), and a
round boss on it: a cylinder of radius 5 on the z axis from z = 4 to z = 8, joined to the plate.
Round both circular edges of the boss to radius 2: the rim of its top face (convex) and its root,
where it meets the plate (concave). Each round takes 2 of the boss's height of 4, so its side
vanishes and the rounds meet on the circle of radius 5 at z = 6: the boss's profile is an S."""
import usd_brep as sm

HALF_SIDE, THICKNESS = 20, 4
BOSS_RADIUS, BOSS_HEIGHT = 5, 4
RADIUS = 2


def build():
    plate = sm.create_box((-HALF_SIDE, -HALF_SIDE, 0), 2 * HALF_SIDE, 2 * HALF_SIDE, THICKNESS)
    boss = sm.create_cylinder((0, 0, THICKNESS), BOSS_RADIUS, BOSS_HEIGHT)
    part = sm.boolean_union(plate, boss)
    # The boss's root and rim are the edges between its side and a flat face, the plate's top or
    # its own; the side's seam has the side on both sides, and the plate's edges have none of it.
    circles = [edge for edge in part.edges()
               if sorted(face.surface().is_planar() for face in edge.faces()) == [False, True]]
    assert len(circles) == 2, f"{len(circles)} circles around the boss"
    return sm.fillet_edges(part, circles, radius=RADIUS)
