"""A square boss rounded at its corners and its root, two concave rounds and one convex at each
corner. A plate, the box from the origin to (40, 30, 5), with a square boss joined to its top: the
box from (12, 7, 5) to (28, 23, 15). The boss's four upright edges are rounded to radius 3, and
then its root, where its sides meet the plate's top, now one closed chain, tangent all along, to
radius 2. The boss's top is left sharp."""
import usd_brep as sm

PLATE = (40, 30, 5)
BOSS_CORNER, BOSS_SIDE, BOSS_HEIGHT = (12, 7, 5), 16, 10
UPRIGHT_RADIUS = 3
ROOT_RADIUS = 2
TOLERANCE = 1e-7


def middle(edge):
    """The point halfway along an edge, by its parameter."""
    low, high = edge.parameter_range()
    return edge.point_at((low + high) / 2)


def build():
    plate = sm.create_box((0, 0, 0), *PLATE)
    boss = sm.create_box(BOSS_CORNER, BOSS_SIDE, BOSS_SIDE, BOSS_HEIGHT)
    part = sm.boolean_union(plate, boss)
    top = PLATE[2] + BOSS_HEIGHT

    # The boss's upright edges are the ones halfway up it.
    uprights = [edge for edge in part.edges()
                if abs(middle(edge)[2] - (PLATE[2] + top) / 2) < TOLERANCE]
    assert len(uprights) == 4, f"{len(uprights)} upright edges"
    sm.fillet_edges(part, uprights, radius=UPRIGHT_RADIUS)

    # The root is the edges lying in the plate's top, z = 5, that are not the plate's own.
    def on_root(edge):
        x, y, z = middle(edge)
        return (abs(z - PLATE[2]) < TOLERANCE
                and all(abs(v.point()[2] - PLATE[2]) < TOLERANCE for v in edge.vertices())
                and TOLERANCE < x < PLATE[0] - TOLERANCE and TOLERANCE < y < PLATE[1] - TOLERANCE)
    root = [edge for edge in part.edges() if on_root(edge)]
    assert len(root) == 8, f"{len(root)} edges around the root"
    return sm.fillet_edges(part, root, radius=ROOT_RADIUS)
