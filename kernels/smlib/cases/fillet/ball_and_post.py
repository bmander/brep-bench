"""A post standing out of a ball, rounded where it leaves. A ball of radius 10 centred on the
origin, and a post: a cylinder of radius 4 on the z axis from z = 0 to z = 15, joined to the ball.
The post leaves the ball on the circle of radius 4 at z = √84. Round that circular edge to radius
2, adding material: a torus about the z axis tangent to the ball and to the post."""
import usd_brep as sm

BALL_RADIUS = 10
POST_RADIUS, POST_HEIGHT = 4, 15
RADIUS = 2


def build():
    ball = sm.create_sphere((0, 0, 0), BALL_RADIUS)
    post = sm.create_cylinder((0, 0, 0), POST_RADIUS, POST_HEIGHT)
    part = sm.boolean_union(ball, post)
    # Where the post leaves the ball is the edge between two curved faces, the ball and the
    # post's side. (A seam lies in one face; the post's top rim has a flat face beside it.)
    def curved_faces(edge):
        return [face for face in edge.faces() if not face.surface().is_planar()]
    junction = [edge for edge in part.edges() if len(curved_faces(edge)) == 2]
    assert junction, "no edge between the ball and the post"
    return sm.fillet_edges(part, junction, radius=RADIUS)
