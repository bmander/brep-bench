"""A hole whose two rim rounds meet at its waist. A square plate from (−10, −10, 0) to
(10, 10, 2), with a hole of radius 3 through it on the z axis. Round both circular edges of the
hole, on the top face and on the bottom face, each to radius 1, half the plate's thickness, so
that the hole's wall vanishes and the rounds meet on the waist circle at z = 1."""
import usd_brep as sm

HALF_SIDE, THICKNESS = 10, 2
HOLE_RADIUS = 3
RADIUS = 1


def build():
    plate = sm.create_box((-HALF_SIDE, -HALF_SIDE, 0), 2 * HALF_SIDE, 2 * HALF_SIDE, THICKNESS)
    # The drill runs past both faces of the plate.
    drill = sm.create_cylinder((0, 0, -1), HOLE_RADIUS, THICKNESS + 2)
    plate = sm.boolean_difference(plate, drill)
    # The hole's two rims are the edges between its wall and a flat face of the plate; the wall's
    # seam has the wall on both sides, and the plate's own edges have none of it.
    rims = [edge for edge in plate.edges()
            if sorted(face.surface().is_planar() for face in edge.faces()) == [False, True]]
    assert len(rims) == 2, f"{len(rims)} rims"
    return sm.fillet_edges(plate, rims, radius=RADIUS)
