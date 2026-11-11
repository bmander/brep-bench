# Volume of { p : d(p, helix arc) <= 2 }: helix radius 1.5, rise 3 a turn, t in [0, 4π].
# For each grid point the nearest arc point is sought by Newton from t = φ + 2πk for every turn k
# (and both ends), φ the point's polar angle: the screw's symmetry puts a local minimum near each.
import numpy as np, sys
r, c, R, T = 1.5, 3/(2*np.pi), 2.0, 4*np.pi
def d2(P, t):
    return (P[:,0]-r*np.cos(t))**2 + (P[:,1]-r*np.sin(t))**2 + (P[:,2]-c*t)**2
def inside(P):
    phi = np.arctan2(P[:,1], P[:,0])
    best = np.minimum(d2(P, np.zeros(len(P))), d2(P, np.full(len(P), T)))
    for k in range(-2, 4):
        t = phi + 2*np.pi*k
        for _ in range(12):
            x, y, z = r*np.cos(t), r*np.sin(t), c*t
            f = -(x-P[:,0])*r*np.sin(t) + (y-P[:,1])*r*np.cos(t) + (z-P[:,2])*c
            fp = r*r + c*c - (x-P[:,0])*r*np.cos(t) - (y-P[:,1])*r*np.sin(t)
            t = np.clip(t - f/np.where(fp > 1e-9, fp, 1e-9), 0, T)
        best = np.minimum(best, d2(P, t))
    return best <= R*R
for h in map(float, sys.argv[1:]):
    xs = np.arange(-3.5 + h/2, 3.5, h)
    X, Y = np.meshgrid(xs, xs)
    XY = np.stack([X.ravel(), Y.ravel()], 1)
    XY = XY[(XY**2).sum(1) <= 3.6**2]
    tot = 0
    for z in np.arange(-2 + h/2, 8, h):
        tot += inside(np.column_stack([XY, np.full(len(XY), z)])).sum()
    print(f"h {h}: volume ≈ {tot*h**3:.3f}", flush=True)
