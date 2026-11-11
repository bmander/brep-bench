"""A tee of two cylinders with its junction rounded. The cross bar, a cylinder of radius 6 about
the x axis from x = −20 to x = 20, joined with the stem, a cylinder of radius 3 about the z axis
from z = 0 to z = 15. Their sides meet in a closed saddle-shaped curve, the junction. Round it to
radius 1: the concave blend a ball of radius 1 sweeps rolling around the junction."""
import usd_brep as sm

BAR_RADIUS, BAR_HALF_LENGTH = 6, 20
STEM_RADIUS, STEM_HEIGHT = 3, 15
RADIUS = 1


def build():
    # A cylinder stands on the z axis: the bar is made there and laid along x.
    bar = sm.create_cylinder((0, 0, -BAR_HALF_LENGTH), BAR_RADIUS, 2 * BAR_HALF_LENGTH)
    sm.rotate(bar, (0, 0, 0), (0, 1, 0), 90)
    stem = sm.create_cylinder((0, 0, 0), STEM_RADIUS, STEM_HEIGHT)
    tee = sm.boolean_union(bar, stem)
    # The junction is the edges between two curved faces, the bar's side and the stem's. (A seam
    # lies in one face; the rims each have a flat end beside them.)
    def curved_faces(edge):
        return [face for face in edge.faces() if not face.surface().is_planar()]
    junction = [edge for edge in tee.edges() if len(curved_faces(edge)) == 2]
    assert junction, "no edge between the bar and the stem"
    return sm.fillet_edges(tee, junction, radius=RADIUS)
