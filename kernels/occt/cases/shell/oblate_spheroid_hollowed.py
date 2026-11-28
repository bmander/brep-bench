"""An oblate spheroid hollowed, its cavity closed. The solid oblate spheroid of
geometry/oblate-spheroid, (x² + y²)/36 + z²/9 ≤ 1, hollowed to a wall 1 thick with no opening: its
surface stays as the outside, and moved 1 inward along its normals (no ellipsoid) it is the inside,
facing in, towards the void."""
from bench import other_case
from shell import hollow

THICKNESS = 1


def build():
    # OCCT builds this hollow (its own measures of it agree with the case's), but writes the
    # cavity's face as an OFFSET_SURFACE of the whole SURFACE_OF_REVOLUTION, bounded by a vertex
    # loop alone: the half ellipse it turns is not in the file, so the whole ellipse turned covers
    # the surface twice, and the offset of the half beyond the axis moves the other way.
    #
    # MakeThickSolid wants an opening; with none, the spheroid's shell and its shrunk copy's,
    # reversed
    return hollow(other_case("geometry/oblate-spheroid").build(), THICKNESS)
