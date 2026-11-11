# The planet-carrier sweep's volume from its definition: 2 × the area of the union of the bar's
# square section over t in [0, 360°] (center (5 cos t, 5 sin t), turned by −2t). Exact polygon
# union of the squares at n instants (and the convex hull of consecutive pairs of each corner's
# step is not taken: the sag between instants shrinks as 1/n², so a few n show the limit).
import math, sys
from shapely.geometry import Polygon
from shapely.ops import unary_union
def square(t):
    c, s = math.cos(-2*t), math.sin(-2*t)
    cx, cy = 5*math.cos(t), 5*math.sin(t)
    return Polygon([(cx + c*x - s*y, cy + s*x + c*y) for x, y in ((-1,-1),(1,-1),(1,1),(-1,1))])
for n in map(int, sys.argv[1:]):
    hull = []
    for i in range(n):
        a, b = square(2*math.pi*i/n), square(2*math.pi*(i+1)/n)
        hull.append(a.union(b).convex_hull)   # an outer bound: each step's swept region within it
    inner = unary_union([square(2*math.pi*i/n) for i in range(n)])
    outer = unary_union(hull)
    print(f"n {n}: volume between {2*inner.area:.5f} and {2*outer.area:.5f}", flush=True)
