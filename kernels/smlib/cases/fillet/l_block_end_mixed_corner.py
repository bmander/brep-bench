"""An L-block's end rounded across its inside corner, one concave round and two convex. The union
of the box from the origin to (20, 10, 6), its foot, and the box from the origin to (6, 10, 20),
its upright. The concave edge from (6, 0, 6) to (6, 10, 6) meets, at its end y = 0, two convex
edges of the end face y = 0: up the upright to (6, 0, 20) and along the foot to (20, 0, 6). All
three are rounded to radius 2, one after another: the concave edge first, and then the chain it
leaves on the end face, up the upright, around the concave round's end and along the foot."""
import usd_brep as sm

FOOT = (20, 10, 6)
UPRIGHT = (6, 10, 20)
RADIUS = 2
TOLERANCE = 1e-7


def middle(edge):
    """The point halfway along an edge, by its parameter."""
    low, high = edge.parameter_range()
    return edge.point_at((low + high) / 2)


def build():
    block = sm.boolean_union(sm.create_box((0, 0, 0), *FOOT), sm.create_box((0, 0, 0), *UPRIGHT))
    # The concave edge, where the upright's face x = 6 meets the foot's top z = 6.
    concave = [edge for edge in block.edges()
               if all(abs(vertex.point()[0] - UPRIGHT[0]) < TOLERANCE
                      and abs(vertex.point()[2] - FOOT[2]) < TOLERANCE
                      for vertex in edge.vertices())]
    assert len(concave) == 1, f"{len(concave)} concave edges"
    sm.fillet_edges(block, concave, radius=RADIUS)

    # The chain the round leaves on the end face y = 0: the edges lying in that face that are
    # not on the block's outside, x = 0 or 20 and z = 0 or 20.
    def on_chain(edge):
        x, y, z = middle(edge)
        return (abs(y) < TOLERANCE and all(abs(v.point()[1]) < TOLERANCE for v in edge.vertices())
                and TOLERANCE < x < FOOT[0] - TOLERANCE and TOLERANCE < z < UPRIGHT[2] - TOLERANCE)
    chain = [edge for edge in block.edges() if on_chain(edge)]
    assert len(chain) == 3, f"{len(chain)} edges in the end face's chain"
    return sm.fillet_edges(block, chain, radius=RADIUS)
