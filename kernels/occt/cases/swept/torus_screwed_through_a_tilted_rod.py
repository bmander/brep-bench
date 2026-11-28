"""A tilted rod grooved by a wheel screwed along it. The rod and groove of
swept/torus-screwed-through-rod, turned rigidly about the origin so that the point that case puts
at (x, y, z) is here at x·u + y·v + z·w, where u = (2, −2, 1)/3, v = (2, 1, −2)/3 and
w = (1, 2, 2)/3.

So: a rod, the cylinder of radius 5 about the line through the origin along w, from the origin to
20w, with a helical groove ground round it by a wheel, a torus whose tube's centres lie on a
circle of radius 6 and whose tube has radius 2, centred at 11.5u, its axis along
cos h·v − sin h·w (h = atan(2π·5.5 / 30)), carried along the screw about w that rises 30 along w
for every whole turn, from a turn of −108° to one of +348°. Cut from the rod everything the wheel
passes through."""
from bench import Refused


def build():
    # As for swept/torus-screwed-through-rod, of which this is a rigid motion: OCCT's one sweep
    # that follows a screw is a pipe along a helix under the Frenet law
    # (BRepOffsetAPI_MakePipe), which takes no solid and stops on the torus's face
    # (BRepFill_Sweep::BuildEdge). Sweeping the torus's face whole would not be enough either:
    # it would first have to be split along its characteristic curve, none of the torus's
    # circles, and the groove's wall is the envelope that curve sweeps out, which would be
    # computed outside the kernel. Turning the axis changes none of that.
    raise Refused("occt has no sweep of a solid along a screw motion: BRepOffsetAPI_MakePipe "
                  "along a helix takes no solid and stops on the torus's face "
                  "(BRepFill_Sweep::BuildEdge), and the groove's wall is the envelope of the "
                  "screwed torus, along a characteristic curve that is none of its circles, "
                  "which occt has no operation to find")
