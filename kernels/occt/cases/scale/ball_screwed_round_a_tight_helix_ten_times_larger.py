"""A ball screwed round a helix tighter than itself, ten times larger. The solid a ball of radius 20
fills as its center runs along the helix of radius 15 about the z axis, from (15, 0, 0), turning
right-handed and rising 30 every turn, for two whole turns, ending at (15, 0, 60).

The helix's radius of curvature, about 16.5, is less than the ball's radius, so the tube about the
helix folds through itself, and its turns, 30 apart, overlap. The solid is everything within 20
of the helix's arc."""
from bench import Refused


def build():
    # As at a tenth of the size (swept/ball-screwed-round-a-tight-helix): the ball's sweep is the
    # tube about the helix with a ball at each end, and the pipe of a circle along the helix
    # (BRepOffsetAPI_MakePipeShell) folds through itself where the helix turns on 16.5, tighter
    # than the ball's 20. OCCT's Booleans take no solid whose boundary passes through itself, and
    # it has no operation that trims a sweep where it folds, nor one that gives everything within
    # a distance of a curve.
    raise Refused("occt has no sweep of a solid, and the tube it would pipe about the helix "
                  "folds through itself where the helix turns tighter than the ball (radius "
                  "16.5 against 20); occt's Booleans take no self-intersecting solid and it has "
                  "no operation to trim a sweep where it folds")
