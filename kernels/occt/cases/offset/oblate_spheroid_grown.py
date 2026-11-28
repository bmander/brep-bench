"""An oblate spheroid grown by 1. The solid oblate spheroid (x² + y²)/36 + z²/9 ≤ 1 centred on the
origin about the z axis (geometry/oblate-spheroid), grown by 1: every point within 1 of it,
bounded by the spheroid's surface moved 1 outward along its normals."""
from bench import other_case
from offset import offset

GROWTH = 1


def build():
    # OCCT grows it right (its own measures are the case's), but its STEP writer writes the grown
    # face, an offset of the whole revolved ellipse, bounded only by a vertex loop at its pole: the
    # file's face is the ellipse's both halves revolved and moved out, the spheroid's surface moved
    # 1 outward and 1 inward, and OCCT reading it back measures that too.
    return offset(other_case("geometry/oblate-spheroid").build(), GROWTH)
