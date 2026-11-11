"""A cube with every edge chamfered. A cube of side 10 between the origin and (10, 10, 10).
Chamfer all twelve of its edges, each with the same distance 2 on both of its faces. Three
chamfers meet at each corner, and the corner is cut flat by a triangle: 26 flat faces."""
import usd_brep as sm

SIDE = 10
DISTANCE = 2
LINEAR = 0   # fillet_edges' flat cross-section: the round's two tangent lines joined by a plane


def build():
    cube = sm.create_box((0, 0, 0), SIDE, SIDE, SIDE)
    # All twelve edges blended at once with the flat section: each a flat cut between the lines
    # where a round of that radius would touch its faces, at a square edge the lines at the
    # radius from it, and each corner closed by its own flat face. (chamfer_fillet, the kernel's
    # chamfer of every edge, mitres the corners instead: chamfer/cube-all-edges-mitered.)
    return sm.fillet_edges(cube, cube.edges(), radius=DISTANCE, xsect_type=LINEAR)
