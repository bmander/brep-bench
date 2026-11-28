"""A ball screwed round a helix tighter than itself, about a tilted axis. The solid of
swept/ball-screwed-round-a-tight-helix, turned rigidly about the origin so that its axis is
w = (1, 2, 2)/3: a ball of radius 2 whose center runs along the helix of radius 1.5 about the line
through the origin along w, from 1.5u = (1, −1, 1/2), turning right-handed about w (from u toward
v) and rising 3 along w every turn, for two whole turns, where u = (2, −2, 1)/3 and
v = (2, 1, −2)/3. Make the solid of everything the ball passes through.

The helix's radius of curvature, about 1.65, is less than the ball's radius, so the envelope of
the balls folds, and the turns, 3 apart, overlap."""
from bench import Refused


def build():
    # As for swept/ball-screwed-round-a-tight-helix, of which this is a rigid motion: the ball's
    # sweep is the tube about the helix with a ball at each end, but the pipe of a circle along
    # the helix (BRepOffsetAPI_MakePipeShell) folds through itself where the helix turns tighter
    # than the ball, and its turns cross one another. OCCT's Booleans take no solid whose
    # boundary passes through itself, and it has no operation that trims a sweep where it folds,
    # nor one that gives everything within a distance of a curve. Turning the axis changes none
    # of that.
    raise Refused("occt has no sweep of a solid, and the tube it would pipe about the helix "
                  "folds through itself where the helix turns tighter than the ball (radius "
                  "1.65 against 2); occt's Booleans take no self-intersecting solid and it has "
                  "no operation to trim a sweep where it folds")
