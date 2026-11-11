"""An L-block whose inside corner is chamfered away. An L-shaped block: the polygon through
(0, 0, 0), (10, 0, 0), (10, 0, 3), (3, 0, 3), (3, 0, 10) and (0, 0, 10) in the xz plane, extruded
along +y from y = 0 to y = 10. Chamfer its one concave edge, the line x = 3, z = 3, with the same
distance 7 on both of its faces, adding material: the chamfer reaches the far edges of both faces
and consumes them."""
import usd_brep as sm

SECTION = [(0, 0), (10, 0), (10, 3), (3, 3), (3, 10), (0, 10)]   # (x, z)
DEPTH = 10
INSIDE = (3, 3)   # (x, z) of the concave edge
DISTANCE = 7
LINEAR = 0   # fillet_edges' flat cross-section: the round's two tangent lines joined by a plane
TOLERANCE = 1e-7


def on_inside_corner(point):
    return abs(point[0] - INSIDE[0]) < TOLERANCE and abs(point[2] - INSIDE[1]) < TOLERANCE


def build():
    corners = [(x, 0, z) for x, z in SECTION]
    outline = [sm.create_line_segment(a, b) for a, b in zip(corners, corners[1:] + corners[:1])]
    block = sm.linear_sweep(outline, (0, 1, 0), DEPTH)
    edges = [edge for edge in block.edges()
             if all(on_inside_corner(vertex.point()) for vertex in edge.vertices())]
    assert len(edges) == 1, f"{len(edges)} edges on the inside corner"
    # A flat face between the lines where a round of that radius would touch the faces: at a
    # square edge, the lines at the radius from it.
    return sm.fillet_edges(block, edges, radius=DISTANCE, xsect_type=LINEAR)
