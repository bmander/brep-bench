"""A ring of twelve balls. Twelve balls of radius 3, their centres evenly spaced on the circle of
radius 10 about the z axis in the plane z = 0, the first at (10, 0, 0). Neighbouring balls
overlap in a lens; no ball meets any other but its two neighbours. Join them, one after another,
into one ring."""
import math

import usd_brep as sm

COUNT = 12
RING = 10
RADIUS = 3


def build():
    balls = [sm.create_sphere((RING * math.cos(2 * math.pi * i / COUNT),
                               RING * math.sin(2 * math.pi * i / COUNT), 0), RADIUS)
             for i in range(COUNT)]
    ring = balls[0]
    for ball in balls[1:]:
        ring = sm.boolean_union(ring, ball)
    return ring
