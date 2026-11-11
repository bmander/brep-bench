"""The mass properties of the solid in a STEP file (ISO 10303-21), from its exact geometry: its
volume, its area, its centre of gravity and its moments of inertia. Standard library only.

    python3 harness/mass.py FILE.step     # prints them as JSON

The harness grades a kernel on the STEP file it writes (SPEC.md, "STEP is the answer"), so the
file's own solid is measured here, by the harness, the same way for every kernel, and not by any
one kernel's reader.

How. By the divergence theorem each measure of the solid is an integral over its boundary of a
polynomial in the position times the outward normal: the volume is ∮ (x·n)/3 dA, its first moment
∫ x dV is ∮ (x²/2) n_x dA, its second moment ∫ x² dV is ∮ (x³/3) n_x dA, and so on. Each face is a
surface S(u, v) trimmed by loops of edges, and its integral ∬ F(S, S_u × S_v) du dv over the
trimmed region of the (u, v) plane is, by Green's theorem, -∮ H du round the loops, where
H(u, v) = ∫ F(u, s) ds from a fixed v_ref to v. Both integrals are taken by Gauss quadrature, the
one round the loops adaptively (Gauss-Kronrod), to about 1e-12 of the solid's size.

The loops are put on the (u, v) plane by inverting the points of each edge's curve in space onto
the face's surface (closed forms for the analytic surfaces, Newton's method on a B-spline), never
by the file's own curves in (u, v), which many writers leave out and some write loosely. A
periodic parameter (an angle) is unwrapped along each loop, so that a loop round a cylinder or a
torus is followed as it winds. Where an edge passes through a singular point of its surface (the
pole of a sphere, the apex of a cone) the loop runs along the singular line in (u, v) from the
direction it arrives in to the one it leaves in. What a periodic face's loops leave open (whether
a face round a sphere holds the pole) is settled by the face's area in (u, v), which must be
positive and no more than the whole surface's.

A face's loops are taken as ISO 10303-42 directs them: anticlockwise seen from where the face's
normal points, the face's normal its surface's turned by `same_sense`. `--loops-by-surface` takes
them by the surface's normal instead, whatever `same_sense` says, as some writers do; the harness
measures a file so only to say why it fails.

Supported: planes, cylinders, cones, spheres, tori (degenerate too), B-spline surfaces (rational
too), surfaces of revolution and of linear extrusion; lines, circles, ellipses, B-spline curves
(rational too), polylines and trimmed curves; MANIFOLD_SOLID_BREP, BREP_WITH_VOIDS and
FACETED_BREP; lengths in any unit, given in millimetres. Anything else is said, as an error.
"""
import json
import math
import re
import sys

sys.path.insert(0, __import__("os").path.dirname(__import__("os").path.abspath(__file__)))
import step as stepfile  # noqa: E402

TAU = 2 * math.pi
# ISO 10303-42 directs a face's loops by the face's normal (anticlockwise seen from where it
# points), the surface's normal turned by same_sense; some writers direct them by the surface's
# normal whatever same_sense says. True measures a file as they mean it: for a diagnosis only.
LOOPS_BY_SURFACE = False


class Unsupported(ValueError):
    """What the file holds that the measurer cannot measure: an entity it does not read yet, or
    geometry it does not make sense of."""


class Invalid(Unsupported):
    """What makes the file no solid at all, whoever reads it: a loop whose edges do not meet."""


# ---------------------------------------------------------------------------------------------
# an entity's arguments

TOKEN = re.compile(r"\s*(#\d+|'(?:[^']|'')*'|\.[A-Za-z_0-9]+\.|[-+]?(?:\d+\.?\d*|\.\d+)"
                   r"(?:[eE][-+]?\d+)?|\$|\*|\(|\)|,|[A-Za-z_][A-Za-z0-9_]*)")


class Ref(int):
    """A reference to another entity, `#12`."""


class Enum(str):
    """An enumeration, `.T.`, `.MILLI.`."""


def tokens(text):
    out, i = [], 0
    while i < len(text):
        m = TOKEN.match(text, i)
        if not m:
            if text[i:].strip() == "":
                break
            raise Unsupported(f"cannot read an entity's arguments: {text[i:i + 40]!r}")
        out.append(m.group(1))
        i = m.end()
    return out


def arguments(text):
    """An entity's arguments, by the name of each of its parts: {NAME: [arguments]}. A typed value
    (`LENGTH_MEASURE(1.)`) is read as its value."""
    toks = tokens(text)
    pos = [0]

    def value():
        t = toks[pos[0]]
        pos[0] += 1
        if t == "(":
            items = []
            if toks[pos[0]] == ")":
                pos[0] += 1
                return items
            while True:
                items.append(value())
                t = toks[pos[0]]
                pos[0] += 1
                if t == ")":
                    return items
                if t != ",":
                    raise Unsupported(f"cannot read an entity's arguments: {text[:60]!r}")
        if t.startswith("#"):
            return Ref(int(t[1:]))
        if t.startswith("'"):
            return t[1:-1]
        if t.startswith("."):
            return Enum(t[1:-1].upper())
        if t in ("$", "*"):
            return None
        if t[0].isalpha() or t[0] == "_":
            # a typed value: NAME(value)
            if pos[0] < len(toks) and toks[pos[0]] == "(":
                pos[0] += 1
                v = value()
                pos[0] += 1  # ')'
                return v
            return t
        return float(t)

    def part():
        name = toks[pos[0]]
        pos[0] += 1
        if toks[pos[0]] != "(":
            raise Unsupported(f"cannot read an entity: {text[:60]!r}")
        return name.upper(), value()

    if toks and toks[0] == "(":
        pos[0] = 1
        parts = {}
        while toks[pos[0]] != ")":
            name, args = part()
            parts[name] = args
        return parts
    name, args = part()
    return {name: args}


# ---------------------------------------------------------------------------------------------
# vectors, as tuples

def add(a, b):
    return (a[0] + b[0], a[1] + b[1], a[2] + b[2])


def sub(a, b):
    return (a[0] - b[0], a[1] - b[1], a[2] - b[2])


def mul(a, s):
    return (a[0] * s, a[1] * s, a[2] * s)


def dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def norm(a):
    return math.sqrt(dot(a, a))


def unit(a):
    n = norm(a)
    if n == 0:
        raise Unsupported("a direction of length zero")
    return mul(a, 1 / n)


def lin(*terms):
    """Σ s·v over (s, v) pairs."""
    x = y = z = 0.0
    for s, v in terms:
        x += s * v[0]
        y += s * v[1]
        z += s * v[2]
    return (x, y, z)


# ---------------------------------------------------------------------------------------------
# quadrature

def legendre(n):
    """Gauss-Legendre nodes and weights on [-1, 1]."""
    xs, ws = [], []
    for i in range(1, n + 1):
        x = math.cos(math.pi * (i - 0.25) / (n + 0.5))
        for _ in range(100):
            p0, p1 = 1.0, x
            for k in range(2, n + 1):
                p0, p1 = p1, ((2 * k - 1) * x * p1 - (k - 1) * p0) / k
            dp = n * (x * p1 - p0) / (x * x - 1)
            dx = p1 / dp
            x -= dx
            if abs(dx) < 1e-16:
                break
        xs.append(x)
        ws.append(2 / ((1 - x * x) * dp * dp))
    return xs, ws


GL = legendre(16)
LEGENDRE = {}


def gauss(n):
    """Gauss-Legendre's n nodes and weights on [-1, 1], kept."""
    if n not in LEGENDRE:
        LEGENDRE[n] = legendre(n)
    return LEGENDRE[n]


def exact(p):
    """How many Gauss points integrate the measures exactly along a polynomial parameter of
    degree p: x³ (degree 3p) times the normal (2p - 1), 5p - 1 in all."""
    return max(2, (5 * p + 1) // 2)


def curved(p):
    """As `exact`, along a parameter in which the surface curves: the area's integrand, the
    normal's length, is the square root of a polynomial, which the polynomial's count leaves
    some 1e-6 short; twelve points a piece take it to well under 1e-12."""
    return max(exact(p), 12)
# Gauss-Kronrod 7-15: the Kronrod nodes (the Gauss ones at odd places) and both weights
XK = [0.991455371120812639206854697526329, 0.949107912342758524526189684047851,
      0.864864423359769072789712788640926, 0.741531185599394439863864773280788,
      0.586087235467691130294144845693013, 0.405845151377397166906606412076961,
      0.207784955007898467600689403773245, 0.0]
WK = [0.022935322010529224963732008058970, 0.063092092629978553290700663189204,
      0.104790010322250183839876322541518, 0.140653259715525918745189590510238,
      0.169004726639267902826583426598550, 0.190350578064785409913256402421014,
      0.204432940075298892414161999234649, 0.209482141084727828012999174891714]
WG = [0.129484966168869693270611432679082, 0.279705391489276667901467771423780,
      0.381830050505118944950369775488975, 0.417959183673469387755102040816327]
KRONROD = []  # (node, Kronrod weight, Gauss weight) on [-1, 1]
for _i, _x in enumerate(XK):
    _g = WG[_i // 2] if _i % 2 else 0.0
    KRONROD.append((-_x, WK[_i], _g))
    if _x:
        KRONROD.append((_x, WK[_i], _g))

# the most points the integral along one piece of a loop may take, before it is given up
BUDGET = 400000

# the measures integrated, in order: the face's area in (u, v), its area, the solid's volume, its
# first moments ∫x, ∫y, ∫z, its second moments ∫x², ∫y², ∫z², ∫xy, ∫yz, ∫zx
NM = 12


def integrand(p, n, c):
    """The measures' integrands at the point `p` of a surface whose normal (S_u × S_v) is `n`,
    about the centre `c`."""
    x, y, z = p[0] - c[0], p[1] - c[1], p[2] - c[2]
    nx, ny, nz = n
    return (1.0, math.sqrt(nx * nx + ny * ny + nz * nz), (x * nx + y * ny + z * nz) / 3,
            x * x * nx / 2, y * y * ny / 2, z * z * nz / 2,
            x * x * x * nx / 3, y * y * y * ny / 3, z * z * z * nz / 3,
            x * x * y * nx / 2, y * y * z * ny / 2, z * z * x * nz / 2)


# ---------------------------------------------------------------------------------------------
# B-splines

def span(n, p, t, U):
    """The knot span of `t`: i with U[i] <= t < U[i+1], p <= i <= n (the NURBS Book, A2.1)."""
    if t >= U[n + 1]:
        return n
    if t <= U[p]:
        return p
    lo, hi = p, n + 1
    mid = (lo + hi) // 2
    while t < U[mid] or t >= U[mid + 1]:
        if t < U[mid]:
            hi = mid
        else:
            lo = mid
        mid = (lo + hi) // 2
    return mid


def basis(i, t, p, U):
    """The non-zero basis functions at `t` and their first derivatives (A2.3, to one derivative)."""
    ndu = [[0.0] * (p + 1) for _ in range(p + 1)]
    ndu[0][0] = 1.0
    left = [0.0] * (p + 1)
    right = [0.0] * (p + 1)
    for j in range(1, p + 1):
        left[j] = t - U[i + 1 - j]
        right[j] = U[i + j] - t
        saved = 0.0
        for r in range(j):
            ndu[j][r] = right[r + 1] + left[j - r]
            tmp = ndu[r][j - 1] / ndu[j][r] if ndu[j][r] else 0.0
            ndu[r][j] = saved + right[r + 1] * tmp
            saved = left[j - r] * tmp
        ndu[j][j] = saved
    N = [ndu[j][p] for j in range(p + 1)]
    if p == 0:
        return N, [0.0]
    D = []
    for r in range(p + 1):
        d = 0.0
        if r >= 1:
            k = ndu[p][r - 1]
            d += ndu[r - 1][p - 1] / k if k else 0.0
        if r <= p - 1:
            k = ndu[p][r]
            d -= ndu[r][p - 1] / k if k else 0.0
        D.append(d * p)
    return N, D


def expand(mults, knots):
    U = []
    for m, k in zip(mults, knots):
        U += [float(k)] * int(m)
    return U


def implicit_knots(name, n, p):
    """The knots of a B-spline whose kind implies them: Bézier, uniform, quasi-uniform."""
    if name.startswith("BEZIER"):
        return [0.0] * (p + 1) + [1.0] * (p + 1)
    if name.startswith("UNIFORM"):
        return [float(i - p) for i in range(n + p + 2)]
    if name.startswith("QUASI_UNIFORM"):
        inner = n - p
        return [0.0] * (p + 1) + [float(i) for i in range(1, inner + 1)] + [float(inner + 1)] * (p + 1)
    raise Unsupported(f"a B-spline without knots: {name}")


# ---------------------------------------------------------------------------------------------
# curves: point(t), deriv(t), invert(p), `period` (or None), `range` (t0, t1), `breaks`

class Line:
    period = None
    range = (-math.inf, math.inf)
    breaks = ()

    def __init__(self, p, d):
        self.p, self.d = p, d
        self.dd = dot(d, d)

    def point(self, t):
        return add(self.p, mul(self.d, t))

    def deriv(self, t):
        return self.d

    def invert(self, q, guess=None):
        return dot(sub(q, self.p), self.d) / self.dd if self.dd else 0.0


class Conic:
    """A circle or an ellipse: O + a cos t X + b sin t Y."""
    period = TAU
    range = (0.0, TAU)
    breaks = ()

    def __init__(self, frame, a, b):
        self.O, self.X, self.Y, _ = frame
        self.a, self.b = a, b

    def point(self, t):
        return lin((1, self.O), (self.a * math.cos(t), self.X), (self.b * math.sin(t), self.Y))

    def deriv(self, t):
        return lin((-self.a * math.sin(t), self.X), (self.b * math.cos(t), self.Y))

    def invert(self, q, guess=None):
        d = sub(q, self.O)
        t = math.atan2(dot(d, self.Y) / self.b, dot(d, self.X) / self.a)
        if self.a != self.b:
            # the nearest point of an ellipse, by Newton from the circle's angle
            for _ in range(20):
                e = sub(self.point(t), q)
                d1 = self.deriv(t)
                d2 = sub(self.O, self.point(t))
                g = dot(e, d1)
                h = dot(d1, d1) + dot(e, d2)
                if not h:
                    break
                dt = g / h
                t -= dt
                if abs(dt) < 1e-15:
                    break
        return t % TAU


class Open:
    """A parabola, O + f (t² X + 2t Y), or a hyperbola, O + a cosh t X + b sinh t Y."""
    period = None
    range = (-math.inf, math.inf)
    breaks = ()

    def __init__(self, frame, kind, a, b=None):
        self.O, self.X, self.Y, _ = frame
        self.kind, self.a, self.b = kind, a, b

    def point(self, t):
        if self.kind == "parabola":
            return lin((1, self.O), (self.a * t * t, self.X), (2 * self.a * t, self.Y))
        return lin((1, self.O), (self.a * math.cosh(t), self.X), (self.b * math.sinh(t), self.Y))

    def deriv(self, t):
        if self.kind == "parabola":
            return lin((2 * self.a * t, self.X), (2 * self.a, self.Y))
        return lin((self.a * math.sinh(t), self.X), (self.b * math.cosh(t), self.Y))

    def invert(self, q, guess=None):
        d = sub(q, self.O)
        y = dot(d, self.Y)
        t = y / (2 * self.a) if self.kind == "parabola" else math.asinh(y / self.b)
        for _ in range(50):
            e = sub(self.point(t), q)
            d1 = self.deriv(t)
            h = dot(d1, d1)
            if not h:
                break
            dt = dot(e, d1) / h
            t -= dt
            if abs(dt) < 1e-15 * (1 + abs(t)):
                break
        return t


class Newton1:
    """Curve inversion by Newton's method, from the nearest of samples or a guess."""

    def samples(self):
        if not hasattr(self, "_samples"):
            ts = []
            for a, b in zip(self.knots, self.knots[1:]):
                ts += [a + (b - a) * k / 4 for k in range(4)]
            ts.append(self.knots[-1])
            self._samples = [(t, self.point(t)) for t in ts]
        return self._samples

    def invert(self, q, guess=None):
        t0, t1 = self.range
        if guess is None:
            t = min(self.samples(), key=lambda s: dot(sub(s[1], q), sub(s[1], q)))[0]
        else:
            t = guess
        for _ in range(50):
            p, d = self.point(t), self.deriv(t)
            e = sub(p, q)
            g = dot(e, d)
            h = dot(d, d)
            if not h:
                break
            dt = g / h
            t2 = t - dt
            if self.period:
                pass
            else:
                t2 = min(max(t2, t0), t1)
            if abs(t2 - t) < 1e-15 * max(1.0, abs(t)):
                t = t2
                break
            t = t2
        return t


class BSplineCurve(Newton1):
    def __init__(self, p, points, weights, U):
        self.p, self.P, self.W, self.U = p, points, weights, U
        self.n = len(points) - 1
        if len(U) != self.n + p + 2:
            raise Unsupported("a B-spline curve whose knots do not match its control points")
        self.range = (U[p], U[self.n + 1])
        self.knots = sorted(set(U[p:self.n + 2]))
        self.breaks = tuple(self.knots[1:-1])
        # a closed curve is followed round as periodic, its join a break (a kink, if it is not
        # smooth there)
        gap_ = norm(sub(self._eval(self.range[0])[0], self._eval(self.range[1])[0]))
        self.closed = gap_ < 1e-9 * (1 + self.size())
        self.period = (self.range[1] - self.range[0]) if self.closed else None
        if self.period:
            t0, P = self.range[0], self.period
            self.breaks = tuple(sorted(set(self.breaks) | {t0 + k * P for k in range(-2, 4)}
                                       | {b + k * P for b in self.breaks for k in (-1, 1, 2)}))

    def size(self):
        return max(norm(sub(a, self.P[0])) for a in self.P)

    def _wrap(self, t):
        if self.period:
            t0 = self.range[0]
            t = t0 + (t - t0) % self.period
        return t

    def _eval(self, t):
        p, U = self.p, self.U
        i = span(self.n, p, t, U)
        N, D = basis(i, t, p, U)
        if self.W is None:
            c = d = (0.0, 0.0, 0.0)
            cx = cy = cz = dx = dy = dz = 0.0
            for j in range(p + 1):
                q = self.P[i - p + j]
                cx += N[j] * q[0]
                cy += N[j] * q[1]
                cz += N[j] * q[2]
                dx += D[j] * q[0]
                dy += D[j] * q[1]
                dz += D[j] * q[2]
            return (cx, cy, cz), (dx, dy, dz)
        ax = ay = az = aw = bx = by = bz = bw = 0.0
        for j in range(p + 1):
            q, w = self.P[i - p + j], self.W[i - p + j]
            nw, dw = N[j] * w, D[j] * w
            ax += nw * q[0]
            ay += nw * q[1]
            az += nw * q[2]
            aw += nw
            bx += dw * q[0]
            by += dw * q[1]
            bz += dw * q[2]
            bw += dw
        c = (ax / aw, ay / aw, az / aw)
        return c, ((bx - bw * c[0]) / aw, (by - bw * c[1]) / aw, (bz - bw * c[2]) / aw)

    def point(self, t):
        return self._eval(self._wrap(t))[0]

    def deriv(self, t):
        return self._eval(self._wrap(t))[1]


class Polyline(Newton1):
    period = None

    def __init__(self, points):
        if len(points) < 2:
            raise Unsupported("a polyline of fewer than two points")
        self.P = points
        self.range = (0.0, float(len(points) - 1))
        self.knots = [float(i) for i in range(len(points))]
        self.breaks = tuple(self.knots[1:-1])

    def _seg(self, t):
        i = min(max(int(math.floor(t)), 0), len(self.P) - 2)
        return i, t - i

    def point(self, t):
        i, s = self._seg(t)
        return add(self.P[i], mul(sub(self.P[i + 1], self.P[i]), s))

    def deriv(self, t):
        i, _ = self._seg(t)
        return sub(self.P[i + 1], self.P[i])

    def invert(self, q, guess=None):
        best = None
        for i in range(len(self.P) - 1):
            a, b = self.P[i], self.P[i + 1]
            d = sub(b, a)
            dd = dot(d, d)
            s = min(max(dot(sub(q, a), d) / dd, 0.0), 1.0) if dd else 0.0
            e = sub(add(a, mul(d, s)), q)
            if best is None or dot(e, e) < best[0]:
                best = (dot(e, e), i + s)
        return best[1]


class Reversed:
    """A curve run the other way: a trimmed curve whose sense is false."""

    def __init__(self, c):
        self.c = c
        self.period = c.period
        self.range = (-c.range[1], -c.range[0])
        self.breaks = tuple(-b for b in c.breaks)

    def point(self, t):
        return self.c.point(-t)

    def deriv(self, t):
        return mul(self.c.deriv(-t), -1)

    def invert(self, q, guess=None):
        return -self.c.invert(q, None if guess is None else -guess)


# ---------------------------------------------------------------------------------------------
# surfaces: ev(u, v) -> (S, S_u, S_v); invert(p, guess) -> (u, v) (guess: (u, v) or None);
# `periods` (pu, pv), each None or the period; `domain` ((u0, u1), (v0, v1)), infinite where
# unbounded; `singular`: [(point, axis 0 or 1, value)], the lines u = value or v = value where
# the surface collapses to a point; `breaks` (u knots, v knots); `polar`: an angle axis (1 or 0)
# whose quadrature is split into panels of no more than a quarter turn, or None.

class Surface:
    periods = (None, None)
    domain = ((-math.inf, math.inf), (-math.inf, math.inf))
    singular = ()
    breaks = ((), ())
    angles = (False, False)
    # the Gauss points for each panel of the inner integral, along each parameter: 16 for an angle
    # (a panel no more than a quarter turn) or a rational piece, fewer where it is a polynomial
    nodes = (16, 16)

    def col(self, u, vs):
        """The points and normals (S_u × S_v) at (u, v) for each v: [(x, y, z, nx, ny, nz)]."""
        out = []
        for v in vs:
            S, Su, Sv = self.ev(u, v)
            out.append((S[0], S[1], S[2], Su[1] * Sv[2] - Su[2] * Sv[1],
                        Su[2] * Sv[0] - Su[0] * Sv[2], Su[0] * Sv[1] - Su[1] * Sv[0]))
        return out

    def row(self, v, us):
        """As `col`, at (u, v) for each u."""
        out = []
        for u in us:
            S, Su, Sv = self.ev(u, v)
            out.append((S[0], S[1], S[2], Su[1] * Sv[2] - Su[2] * Sv[1],
                        Su[2] * Sv[0] - Su[0] * Sv[2], Su[0] * Sv[1] - Su[1] * Sv[0]))
        return out


class Plane(Surface):
    nodes = (exact(1), exact(1))

    def __init__(self, frame):
        self.O, self.X, self.Y, self.Z = frame

    def ev(self, u, v):
        return lin((1, self.O), (u, self.X), (v, self.Y)), self.X, self.Y

    def col(self, u, vs):
        O, X, Y, Z = self.O, self.X, self.Y, self.Z
        return [(O[0] + u * X[0] + v * Y[0], O[1] + u * X[1] + v * Y[1],
                 O[2] + u * X[2] + v * Y[2], Z[0], Z[1], Z[2]) for v in vs]

    def row(self, v, us):
        O, X, Y, Z = self.O, self.X, self.Y, self.Z
        return [(O[0] + u * X[0] + v * Y[0], O[1] + u * X[1] + v * Y[1],
                 O[2] + u * X[2] + v * Y[2], Z[0], Z[1], Z[2]) for u in us]

    def invert(self, q, guess=None):
        d = sub(q, self.O)
        return dot(d, self.X), dot(d, self.Y)


class Revolved(Surface):
    """A surface of a frame's revolution, O + ρ(v)(cos u X + sin u Y) + h(v) Z, u an angle."""
    periods = (TAU, None)
    angles = (True, False)

    def ev(self, u, v):
        r, h, dr, dh = self.profile(v)
        cu, su = math.cos(u), math.sin(u)
        radial = lin((cu, self.X), (su, self.Y))
        S = lin((1, self.O), (r, radial), (h, self.Z))
        Su = lin((-r * su, self.X), (r * cu, self.Y))
        Sv = lin((dr, radial), (dh, self.Z))
        return S, Su, Sv

    # S = O + ρ(v) e + h(v) Z, e = cos u X + sin u Y; S_u × S_v = ρ (h' e - ρ' Z)
    def col(self, u, vs):
        c, s = math.cos(u), math.sin(u)
        O, X, Y, Z = self.O, self.X, self.Y, self.Z
        ex, ey, ez = c * X[0] + s * Y[0], c * X[1] + s * Y[1], c * X[2] + s * Y[2]
        out = []
        for v in vs:
            r, h, dr, dh = self.profile(v)
            out.append((O[0] + r * ex + h * Z[0], O[1] + r * ey + h * Z[1],
                        O[2] + r * ez + h * Z[2], r * (dh * ex - dr * Z[0]),
                        r * (dh * ey - dr * Z[1]), r * (dh * ez - dr * Z[2])))
        return out

    def row(self, v, us):
        r, h, dr, dh = self.profile(v)
        O, X, Y, Z = self.O, self.X, self.Y, self.Z
        out = []
        for u in us:
            c, s = math.cos(u), math.sin(u)
            ex, ey, ez = c * X[0] + s * Y[0], c * X[1] + s * Y[1], c * X[2] + s * Y[2]
            out.append((O[0] + r * ex + h * Z[0], O[1] + r * ey + h * Z[1],
                        O[2] + r * ez + h * Z[2], r * (dh * ex - dr * Z[0]),
                        r * (dh * ey - dr * Z[1]), r * (dh * ez - dr * Z[2])))
        return out

    def polar(self, q):
        d = sub(q, self.O)
        x, y, z = dot(d, self.X), dot(d, self.Y), dot(d, self.Z)
        return math.atan2(y, x), math.hypot(x, y), z


class Cylinder(Revolved):
    nodes = (16, exact(1))

    def __init__(self, frame, r):
        self.O, self.X, self.Y, self.Z = frame
        self.r = r

    def profile(self, v):
        return self.r, v, 0.0, 1.0

    def invert(self, q, guess=None):
        u, _, z = self.polar(q)
        return u, z


class Cone(Revolved):
    nodes = (16, exact(1))

    def __init__(self, frame, r, angle):
        self.O, self.X, self.Y, self.Z = frame
        self.r, self.k = r, math.tan(angle)
        if not self.k:
            raise Unsupported("a cone of semi-angle zero")
        apex = -r / self.k
        self.singular = ((add(self.O, mul(self.Z, apex)), 1, apex),)

    def profile(self, v):
        return self.r + v * self.k, v, self.k, 1.0

    def invert(self, q, guess=None):
        u, rho, z = self.polar(q)
        # the side of the apex the point is on: its radius there is negative, its angle turned
        if self.r + z * self.k < 0:
            u += math.pi
        return u, z


class Sphere(Revolved):
    domain = ((-math.inf, math.inf), (-math.pi / 2, math.pi / 2))
    angles = (True, True)

    def __init__(self, frame, r):
        self.O, self.X, self.Y, self.Z = frame
        self.r = r
        self.singular = ((add(self.O, mul(self.Z, -r)), 1, -math.pi / 2),
                         (add(self.O, mul(self.Z, r)), 1, math.pi / 2))

    def profile(self, v):
        c, s = math.cos(v), math.sin(v)
        return self.r * c, self.r * s, -self.r * s, self.r * c

    def invert(self, q, guess=None):
        u, rho, z = self.polar(q)
        return u, math.atan2(z, rho)


class Torus(Revolved):
    periods = (TAU, TAU)
    angles = (True, True)

    def __init__(self, frame, R, r, outer=None):
        self.O, self.X, self.Y, self.Z = frame
        self.R, self.r, self.outer = R, r, outer
        sing = []
        if r >= R:
            # the surface meets its axis
            a = math.acos(-R / r)
            h = r * math.sin(a)
            sing = [(add(self.O, mul(self.Z, h)), 1, a), (add(self.O, mul(self.Z, -h)), 1, -a)]
        self.singular = tuple(sing)

    def profile(self, v):
        c, s = math.cos(v), math.sin(v)
        return self.R + self.r * c, self.r * s, -self.r * s, self.r * c

    def invert(self, q, guess=None):
        u, rho, z = self.polar(q)
        apple = (u, math.atan2(z, rho - self.R))
        if self.r < self.R or self.outer is True:
            return apple
        # within the axis: the radius R + r cos v is negative there, the angle turned
        lemon = (u + math.pi, math.atan2(z, -rho - self.R))
        if self.outer is False:
            return lemon
        # the sheet the point is on: (ρ - R)² + z² = r² on the apple, (ρ + R)² + z² = r² within
        off_apple = abs(math.hypot(rho - self.R, z) - self.r)
        off_lemon = abs(math.hypot(rho + self.R, z) - self.r)
        if abs(off_apple - off_lemon) > 1e-9 * self.r or guess is None:
            return apple if off_apple <= off_lemon else lemon

        def off(a):
            du = (a[0] - guess[0] + math.pi) % TAU - math.pi
            dv = (a[1] - guess[1] + math.pi) % TAU - math.pi
            return du * du + dv * dv
        return min((apple, lemon), key=off)


class BSplineSurface(Surface):
    def __init__(self, pu, pv, grid, weights, U, V):
        self.pu, self.pv, self.P, self.W, self.U, self.V = pu, pv, grid, weights, U, V
        self.nu, self.nv = len(grid) - 1, len(grid[0]) - 1
        if len(U) != self.nu + pu + 2 or len(V) != self.nv + pv + 2:
            raise Unsupported("a B-spline surface whose knots do not match its control points")
        u0, u1 = U[pu], U[self.nu + 1]
        v0, v1 = V[pv], V[self.nv + 1]
        self.domain = ((u0, u1), (v0, v1))
        ku = sorted(set(U[pu:self.nu + 2]))
        kv = sorted(set(V[pv:self.nv + 2]))
        self.knots = (ku, kv)
        self.breaks = (tuple(ku[1:-1]), tuple(kv[1:-1]))
        pts = [q for row in grid for q in row]
        lo = [min(q[i] for q in pts) for i in range(3)]
        hi = [max(q[i] for q in pts) for i in range(3)]
        self.size = norm(sub(tuple(hi), tuple(lo))) or 1.0
        eps = 1e-9 * self.size
        # closed where opposite boundaries meet; periodic for the unwrapping of loops
        per = []
        for axis in (0, 1):
            (a0, a1), (b0, b1) = self.domain[axis], self.domain[1 - axis]
            ok = True
            for k in range(9):
                b = b0 + (b1 - b0) * k / 8
                pa = self._ev(a0, b)[0] if axis == 0 else self._ev(b, a0)[0]
                pb = self._ev(a1, b)[0] if axis == 0 else self._ev(b, a1)[0]
                if norm(sub(pa, pb)) > eps:
                    ok = False
                    break
            per.append(a1 - a0 if ok else None)
        self.periods = tuple(per)
        # collapsed boundaries: a pole
        sing = []
        for axis in (0, 1):
            for end in self.domain[axis]:
                (b0, b1) = self.domain[1 - axis]
                ps = [self._ev(end, b0 + (b1 - b0) * k / 8)[0] if axis == 0 else
                      self._ev(b0 + (b1 - b0) * k / 8, end)[0] for k in range(9)]
                if max(norm(sub(q, ps[0])) for q in ps) < eps:
                    sing.append((ps[0], axis, end))
        self.singular = tuple(sing)
        # samples for a first guess at inversion
        su = [a + (b - a) * k / 4 for a, b in zip(ku, ku[1:]) for k in range(4)] + [ku[-1]]
        sv = [a + (b - a) * k / 4 for a, b in zip(kv, kv[1:]) for k in range(4)] + [kv[-1]]
        while len(su) * len(sv) > 40000:
            su, sv = (su[::2] if len(su) > len(sv) else su), (sv[::2] if len(sv) >= len(su) else sv)
        self.samples = [((u, v), self._ev(u, v)[0]) for u in su for v in sv]
        # the basis at the points the quadrature visits again and again (whole knot spans)
        self._basis = ({}, {})
        # a polynomial along a parameter where the weights do not vary along it
        def flat(axis):
            if weights is None:
                return True
            rows = weights if axis == 1 else list(zip(*weights))
            return all(max(r) - min(r) <= 1e-12 * max(r) for r in rows)
        self.nodes = (curved(pu) if flat(0) else 16, curved(pv) if flat(1) else 16)

    def _wrap(self, u, v):
        if self.periods[0]:
            a = self.domain[0][0]
            u = a + (u - a) % self.periods[0]
        if self.periods[1]:
            a = self.domain[1][0]
            v = a + (v - a) % self.periods[1]
        return u, v

    def _ev(self, u, v):
        pu, pv = self.pu, self.pv
        i = span(self.nu, pu, u, self.U)
        j = span(self.nv, pv, v, self.V)
        Nu, Du = basis(i, u, pu, self.U)
        Nv, Dv = basis(j, v, pv, self.V)
        rational = self.W is not None
        s = [0.0] * 4
        su = [0.0] * 4
        sv = [0.0] * 4
        for a in range(pu + 1):
            row = self.P[i - pu + a]
            wrow = self.W[i - pu + a] if rational else None
            na, da = Nu[a], Du[a]
            for b in range(pv + 1):
                q = row[j - pv + b]
                w = wrow[j - pv + b] if rational else 1.0
                nb, db = Nv[b], Dv[b]
                f, fu, fv = na * nb * w, da * nb * w, na * db * w
                s[0] += f * q[0]
                s[1] += f * q[1]
                s[2] += f * q[2]
                s[3] += f
                su[0] += fu * q[0]
                su[1] += fu * q[1]
                su[2] += fu * q[2]
                su[3] += fu
                sv[0] += fv * q[0]
                sv[1] += fv * q[1]
                sv[2] += fv * q[2]
                sv[3] += fv
        w = s[3]
        S = (s[0] / w, s[1] / w, s[2] / w)
        Su = ((su[0] - su[3] * S[0]) / w, (su[1] - su[3] * S[1]) / w, (su[2] - su[3] * S[2]) / w)
        Sv = ((sv[0] - sv[3] * S[0]) / w, (sv[1] - sv[3] * S[1]) / w, (sv[2] - sv[3] * S[2]) / w)
        return S, Su, Sv

    def ev(self, u, v):
        return self._ev(*self._wrap(u, v))

    # along a column (u fixed) the u basis is the same at every point: the net is collapsed once to
    # a curve in v and its derivative in u, homogeneous, and each point costs a curve's sum
    def _line(self, axis, t, ss):
        """[(x, y, z, nx, ny, nz)] at parameter `t` of `axis` fixed, each s of the other."""
        per = self.periods[axis]
        a0 = self.domain[axis][0]
        if per:
            t = a0 + (t - a0) % per
        p, n, K = (self.pu, self.nu, self.U) if axis == 0 else (self.pv, self.nv, self.V)
        q, m, L = (self.pv, self.nv, self.V) if axis == 0 else (self.pu, self.nu, self.U)
        i = span(n, p, t, K)
        N, D = basis(i, t, p, K)
        W, P = self.W, self.P
        b0 = self.domain[1 - axis][0]
        per2 = self.periods[1 - axis]
        cache = self._basis[1 - axis]
        if len(cache) > 100000:
            cache.clear()
        # the basis at each point first, so that only the control points they touch are collapsed
        hits = []
        for s in ss:
            if per2:
                s = b0 + (s - b0) % per2
            hit = cache.get(s)
            if hit is None:
                j = span(m, q, s, L)
                hit = cache[s] = (j,) + basis(j, s, q, L)
            hits.append(hit)
        lo = min(h[0] for h in hits) - q if hits else 0
        hi = max(h[0] for h in hits) if hits else -1
        Q, R = {}, {}
        for j in range(lo, hi + 1):
            qx = qy = qz = qw = rx = ry = rz = rw = 0.0
            for a in range(p + 1):
                ii = i - p + a
                pt = P[ii][j] if axis == 0 else P[j][ii]
                w = (W[ii][j] if axis == 0 else W[j][ii]) if W is not None else 1.0
                nw, dw = N[a] * w, D[a] * w
                qx += nw * pt[0]
                qy += nw * pt[1]
                qz += nw * pt[2]
                qw += nw
                rx += dw * pt[0]
                ry += dw * pt[1]
                rz += dw * pt[2]
                rw += dw
            Q[j] = (qx, qy, qz, qw)
            R[j] = (rx, ry, rz, rw)
        out = []
        for j, Nb, Db in hits:
            sx = sy = sz = sw = tx = ty = tz = tw = ux = uy = uz = uw = 0.0
            for b in range(q + 1):
                qq, rr = Q[j - q + b], R[j - q + b]
                nb, db = Nb[b], Db[b]
                sx += nb * qq[0]
                sy += nb * qq[1]
                sz += nb * qq[2]
                sw += nb * qq[3]
                tx += nb * rr[0]  # along the fixed axis
                ty += nb * rr[1]
                tz += nb * rr[2]
                tw += nb * rr[3]
                ux += db * qq[0]  # along the free axis
                uy += db * qq[1]
                uz += db * qq[2]
                uw += db * qq[3]
            x, y, z = sx / sw, sy / sw, sz / sw
            ax, ay, az = (tx - tw * x) / sw, (ty - tw * y) / sw, (tz - tw * z) / sw
            bx, by, bz = (ux - uw * x) / sw, (uy - uw * y) / sw, (uz - uw * z) / sw
            if axis == 0:  # S_u = a, S_v = b
                out.append((x, y, z, ay * bz - az * by, az * bx - ax * bz, ax * by - ay * bx))
            else:          # S_u = b, S_v = a
                out.append((x, y, z, by * az - bz * ay, bz * ax - bx * az, bx * ay - by * ax))
        return out

    def col(self, u, vs):
        return self._line(0, u, vs)

    def row(self, v, us):
        return self._line(1, v, us)

    def nearest_sample(self, q):
        return min(self.samples, key=lambda s: dot(sub(s[1], q), sub(s[1], q)))[0]

    def invert(self, q, guess=None):
        u, v = self.nearest_sample(q) if guess is None else self._wrap(*guess)
        uv = newton2(self, q, u, v)
        if guess is not None and norm(sub(self._ev(*self._wrap(*uv))[0], q)) > 1e-4 * self.size:
            # Newton from the guess found another fold of the surface: from the nearest sample
            other = newton2(self, q, *self.nearest_sample(q))
            if norm(sub(self._ev(*self._wrap(*other))[0], q)) < \
                    norm(sub(self._ev(*self._wrap(*uv))[0], q)):
                return other
        return uv


def newton2(s, q, u, v):
    """The (u, v) of the point of `s` nearest `q`, by Newton's method from (u, v), kept within
    the surface's bounded parameters."""
    (u0, u1), (v0, v1) = s.domain
    for _ in range(60):
        S, Su, Sv = s.ev(u, v)
        e = sub(q, S)
        E, F, G = dot(Su, Su), dot(Su, Sv), dot(Sv, Sv)
        a, b = dot(Su, e), dot(Sv, e)
        det = E * G - F * F
        if det <= 1e-300:
            # at a pole: step along the parameter that moves the point
            if E > G and E:
                du, dv = a / E, 0.0
            elif G:
                du, dv = 0.0, b / G
            else:
                break
        else:
            du, dv = (G * a - F * b) / det, (E * b - F * a) / det
        nu, nv = u + du, v + dv
        if not s.periods[0]:
            nu = min(max(nu, u0), u1)
        if not s.periods[1]:
            nv = min(max(nv, v0), v1)
        done = abs(nu - u) < 1e-15 * (1 + abs(u)) and abs(nv - v) < 1e-15 * (1 + abs(v))
        u, v = nu, nv
        if done:
            break
    return u, v


class Swept(Surface):
    """A surface of revolution, σ(u, v) = C + (λ(v) - C) turned by the angle u about the axis
    (ISO 10303-42's: u the angle, v the swept curve's parameter), or of linear extrusion,
    σ(u, v) = λ(u) + v V (u the curve's, v along V)."""

    def __init__(self, curve, revolve=None, extrude=None):
        self.c = curve
        self.revolve, self.extrude = revolve, extrude
        t0, t1 = curve.range
        along = curved(curve.p) if isinstance(curve, BSplineCurve) and curve.W is None else \
            exact(1) if isinstance(curve, (Line, Polyline)) else 16
        if revolve:
            self.A0, self.A = revolve
            if isinstance(curve, Line) and abs(dot(cross(curve.d, self.A), sub(curve.p, self.A0))) \
                    > 1e-12 * norm(curve.d) * (1 + norm(sub(curve.p, self.A0))):
                along = 16  # a line askew of the axis turns a hyperboloid: its radius not linear
            self.periods = (TAU, curve.period)
            self.domain = ((-math.inf, math.inf), (t0, t1))
            self.breaks = ((), tuple(curve.breaks))
            # a conic's parameter is an angle: its panels no more than a quarter turn
            self.angles = (True, isinstance(curve, Conic))
            self.nodes = (16, along)
            self.singular = tuple(self._on_axis())
            # a frame about the axis, for inversion
            X = cross(self.A, (1, 0, 0)) if abs(self.A[0]) < 0.9 else cross(self.A, (0, 1, 0))
            self.X = unit(X)
            self.Y = cross(self.A, self.X)
            self.mirrored = self._mirrored()
        else:
            self.D = extrude
            self.periods = (curve.period, None)
            self.domain = ((t0, t1), (-math.inf, math.inf))
            self.breaks = (tuple(curve.breaks), ())
            self.angles = (isinstance(curve, Conic), False)
            self.nodes = (along, exact(1))
        self.samples = None

    def _on_axis(self):
        """Where the profile meets the axis: the poles, lines v = t of the surface."""
        c = self.c
        t0, t1 = c.range
        if isinstance(c, Line):
            # a line meets the axis at most once: where it comes nearest
            w0 = sub(c.p, self.A0)
            dd = sub(c.d, mul(self.A, dot(c.d, self.A)))
            ww = sub(w0, mul(self.A, dot(w0, self.A)))
            if not dot(dd, dd):
                return []
            t = -dot(ww, dd) / dot(dd, dd)
            if norm(add(ww, mul(dd, t))) < 1e-9 * (1 + norm(w0)):
                return [(c.point(t), 1, t)]
            return []
        if not (math.isfinite(t0) and math.isfinite(t1)):
            # a parabola's or a hyperbola's parameter: its turns near the vertex are what meet
            # the axis
            t0, t1 = -100.0, 100.0

        def r(t):
            w = sub(c.point(t), self.A0)
            return norm(sub(w, mul(self.A, dot(w, self.A))))
        cuts = sorted({t0, t1, *(b for b in c.breaks if t0 < b < t1)})
        n = 16 if math.isfinite(c.range[0]) else 4000
        ts = [a + (b - a) * k / n for a, b in zip(cuts, cuts[1:]) for k in range(n)] + [t1]
        size = max(norm(sub(c.point(t), self.A0)) for t in ts) or 1.0
        rs = [r(t) for t in ts]
        found = []
        for k, t in enumerate(ts):
            lo = rs[k - 1] if k else math.inf
            hi = rs[k + 1] if k + 1 < len(ts) else math.inf
            if rs[k] <= lo and rs[k] <= hi:
                a, b = ts[max(k - 1, 0)], ts[min(k + 1, len(ts) - 1)]
                for _ in range(100):
                    m1, m2 = b - (b - a) * 0.6180339887498949, a + (b - a) * 0.6180339887498949
                    if r(m1) < r(m2):
                        b = m2
                    else:
                        a = m1
                tm = (a + b) / 2
                if r(tm) < 1e-9 * size and all(abs(tm - f[2]) > 1e-9 * (t1 - t0) for f in found):
                    found.append((c.point(tm), 1, tm))
        return found

    def _mirrored(self):
        """Whether the profile is its own mirror image across the axis, so that turning either
        side of it makes the same surface (an ellipse about its axis, not a circle beside it)."""
        c = self.c
        t0, t1 = c.range
        if not (math.isfinite(t0) and math.isfinite(t1)):
            return True
        for k in range(16):
            p = c.point(t0 + (t1 - t0) * (k + 0.5) / 16)
            q = add(self.A0, self._rot(sub(p, self.A0), math.pi))
            if norm(sub(c.point(c.invert(q)), q)) > 1e-9 * (1 + norm(sub(p, self.A0))):
                return False
        return True

    def _rot(self, w, a):
        c, s = math.cos(a), math.sin(a)
        A = self.A
        return lin((c, w), (s, cross(A, w)), ((1 - c) * dot(A, w), A))

    def ev(self, u, v):
        if self.revolve:
            p, d = self.c.point(v), self.c.deriv(v)
            w = self._rot(sub(p, self.A0), u)
            return add(self.A0, w), cross(self.A, w), self._rot(d, u)
        p, d = self.c.point(u), self.c.deriv(u)
        return add(p, mul(self.D, v)), d, self.D

    def _guess(self, q):
        if self.revolve:
            # the point turned about the axis into the profile's half-plane, and found on the
            # profile there
            c = self.c
            # the half-plane of the profile's start (or of a point of it off the axis): the same
            # for every point, so that a face's loops are found on one sheet of the surface
            t0, t1 = c.range
            for t in ((t0, (t0 + t1) / 2) if math.isfinite(t0) else (1.0, -1.0, 10.0)):
                w = sub(c.point(t), self.A0)
                radial = sub(w, mul(self.A, dot(w, self.A)))
                if norm(radial) > 1e-9 * (1 + norm(w)):
                    break
            e = sub(q, self.A0)
            h = dot(e, self.A)
            rho = norm(sub(e, mul(self.A, h)))
            if norm(radial) > 0:
                p = add(add(self.A0, mul(self.A, h)), mul(unit(radial), rho))
                t = c.invert(p)
                w = sub(c.point(t), self.A0)
                a1 = math.atan2(dot(w, self.Y), dot(w, self.X))
                a2 = math.atan2(dot(e, self.Y), dot(e, self.X))
                return a2 - a1, t
        elif not math.isfinite(self.c.range[0]):
            # a line's or a conic's parameter: the point moved back along the direction
            e = sub(q, mul(self.D, dot(sub(q, self.c.point(0.0)), self.D) / dot(self.D, self.D)))
            t = self.c.invert(e)
            return t, dot(sub(q, self.c.point(t)), self.D) / dot(self.D, self.D)
        t0, t1 = self.c.range
        if self.samples is None:
            ts = sorted({t0, t1, *self.c.breaks} & {t for t in (t0, t1, *self.c.breaks)
                                                       if t0 <= t <= t1})
            self.samples = [a + (b - a) * k / 8 for a, b in zip(ts, ts[1:]) for k in range(8)] + [t1]
        best = None
        for t in self.samples:
            p = self.c.point(t)
            if self.revolve:
                w, e = sub(p, self.A0), sub(q, self.A0)
                h1, h2 = dot(w, self.A), dot(e, self.A)
                r1 = norm(sub(w, mul(self.A, h1)))
                r2 = norm(sub(e, mul(self.A, h2)))
                d = (r1 - r2) ** 2 + (h1 - h2) ** 2
            else:
                e = sub(q, p)
                e = sub(e, mul(self.D, dot(e, self.D) / dot(self.D, self.D)))
                d = dot(e, e)
            if best is None or d < best[0]:
                best = (d, t)
        t = best[1]
        if self.revolve:
            w, e = sub(self.c.point(t), self.A0), sub(q, self.A0)
            a1 = math.atan2(dot(w, self.Y), dot(w, self.X))
            a2 = math.atan2(dot(e, self.Y), dot(e, self.X))
            return a2 - a1, t
        return t, dot(sub(q, self.c.point(t)), self.D) / dot(self.D, self.D)

    def invert(self, q, guess=None):
        u, v = self._guess(q) if guess is None else guess
        return newton2(self, q, u, v)


# ---------------------------------------------------------------------------------------------
# reading the file

class Model:
    def __init__(self, text):
        self.schema, self.entities = stepfile.parse(text)
        self._args = {}
        self._geom = {}
        self.length = 1.0   # millimetres per the file's unit of length
        self.angle = 1.0    # radians per the file's unit of plane angle
        self._units()

    def args(self, ref):
        a = self._args.get(ref)
        if a is None:
            e = self.entities.get(ref)
            if e is None:
                raise Unsupported(f"#{ref} is named but not defined")
            a = arguments(e.text)
            self._args[ref] = a
        return a

    def one(self, ref):
        """A simple entity's name and arguments."""
        a = self.args(ref)
        if len(a) != 1:
            return None, a
        (name, args), = a.items()
        return name, args

    def _units(self):
        for ref, e in self.entities.items():
            if "LENGTH_UNIT" in e.names:
                self.length = self._unit(ref, "length")
            if "PLANE_ANGLE_UNIT" in e.names:
                self.angle = self._unit(ref, "angle")

    def _unit(self, ref, what):
        a = self.args(ref)
        if "SI_UNIT" in a:
            prefix, name = a["SI_UNIT"][-2:] if len(a["SI_UNIT"]) >= 2 else (None, a["SI_UNIT"][0])
            scale = {None: 1.0, "MILLI": 1e-3, "CENTI": 1e-2, "DECI": 1e-1, "KILO": 1e3,
                     "MICRO": 1e-6, "NANO": 1e-9}.get(prefix)
            if scale is None:
                raise Unsupported(f"a unit prefixed {prefix}")
            if what == "length":
                return scale * 1000.0  # metres to millimetres
            return scale
        if "CONVERSION_BASED_UNIT" in a:
            measure = a["CONVERSION_BASED_UNIT"][1]
            ma = self.args(measure)
            for k, v in ma.items():
                if k.endswith("MEASURE_WITH_UNIT"):
                    return float(v[0]) * self._unit(v[1], what)
        raise Unsupported(f"a unit of {what} it does not know: #{ref}")

    # -- points, frames

    def point(self, ref):
        name, a = self.one(ref)
        if name == "CARTESIAN_POINT":
            c = [float(x) * self.length for x in a[1]]
            return tuple(c + [0.0] * (3 - len(c)))
        if name == "VERTEX_POINT":
            return self.point(a[1])
        raise Unsupported(f"#{ref} is not a point ({name})")

    def direction(self, ref):
        name, a = self.one(ref)
        if name != "DIRECTION":
            raise Unsupported(f"#{ref} is not a direction ({name})")
        c = [float(x) for x in a[1]]
        return tuple(c + [0.0] * (3 - len(c)))

    def frame(self, ref):
        name, a = self.one(ref)
        if name != "AXIS2_PLACEMENT_3D":
            raise Unsupported(f"a placement {name}")
        O = self.point(a[1])
        Z = unit(self.direction(a[2])) if a[2] is not None else (0.0, 0.0, 1.0)
        X = self.direction(a[3]) if len(a) > 3 and a[3] is not None else \
            ((1.0, 0.0, 0.0) if abs(Z[0]) < 0.9 else (0.0, 1.0, 0.0))
        X = unit(sub(X, mul(Z, dot(X, Z))))
        return O, X, cross(Z, X), Z

    # -- curves

    def curve(self, ref):
        if ref in self._geom:
            return self._geom[ref]
        c = self._curve(ref)
        self._geom[ref] = c
        return c

    def _curve(self, ref):
        a = self.args(ref)
        names = set(a)
        L = self.length
        if "LINE" in names:
            _, p, v = a["LINE"]
            vn, va = self.one(v)
            if vn != "VECTOR":
                raise Unsupported(f"a line's direction {vn}")
            return Line(self.point(p), mul(unit(self.direction(va[1])), float(va[2]) * L))
        if "CIRCLE" in names:
            _, f, r = a["CIRCLE"]
            return Conic(self.frame(f), float(r) * L, float(r) * L)
        if "ELLIPSE" in names:
            _, f, r1, r2 = a["ELLIPSE"]
            return Conic(self.frame(f), float(r1) * L, float(r2) * L)
        if "PARABOLA" in names:
            _, f, focal = a["PARABOLA"]
            return Open(self.frame(f), "parabola", float(focal) * L)
        if "HYPERBOLA" in names:
            _, f, r1, r2 = a["HYPERBOLA"]
            return Open(self.frame(f), "hyperbola", float(r1) * L, float(r2) * L)
        if names & {"SURFACE_CURVE", "SEAM_CURVE", "INTERSECTION_CURVE"}:
            n = (names & {"SURFACE_CURVE", "SEAM_CURVE", "INTERSECTION_CURVE"}).pop()
            return self.curve(a[n][1])
        if "TRIMMED_CURVE" in names:
            args = a["TRIMMED_CURVE"]
            basis = self.curve(args[1])
            return basis if args[4] != "F" else Reversed(basis)
        if "POLYLINE" in names:
            return Polyline([self.point(p) for p in a["POLYLINE"][1]])
        bs = [n for n in names if n.startswith(("B_SPLINE_CURVE", "BEZIER_CURVE", "UNIFORM_CURVE",
                                                "QUASI_UNIFORM_CURVE"))]
        if bs:
            return self._bspline_curve(a)
        raise Unsupported(f"a curve {'/'.join(sorted(names))}")

    def _bspline_curve(self, a):
        if len(a) == 1:
            (name, args), = a.items()
            p, pts = int(args[1]), args[2]
            if name == "B_SPLINE_CURVE_WITH_KNOTS":
                U = expand(args[6], args[7])
            else:
                U = implicit_knots(name, len(pts) - 1, int(args[1]))
            W = None
        else:
            base = a["B_SPLINE_CURVE"]
            p, pts = int(base[0]), base[1]
            if "B_SPLINE_CURVE_WITH_KNOTS" in a:
                k = a["B_SPLINE_CURVE_WITH_KNOTS"]
                U = expand(k[0], k[1])
            else:
                kind = next(n for n in a if n in ("BEZIER_CURVE", "UNIFORM_CURVE",
                                                  "QUASI_UNIFORM_CURVE"))
                U = implicit_knots(kind, len(pts) - 1, p)
            W = [float(w) for w in a["RATIONAL_B_SPLINE_CURVE"][0]] \
                if "RATIONAL_B_SPLINE_CURVE" in a else None
        return BSplineCurve(p, [self.point(q) for q in pts], W, U)

    # -- surfaces

    def surface(self, ref):
        if ref in self._geom:
            return self._geom[ref]
        s = self._surface(ref)
        self._geom[ref] = s
        return s

    def _surface(self, ref):
        a = self.args(ref)
        names = set(a)
        L = self.length
        if "PLANE" in names:
            return Plane(self.frame(a["PLANE"][1]))
        if "CYLINDRICAL_SURFACE" in names:
            _, f, r = a["CYLINDRICAL_SURFACE"]
            return Cylinder(self.frame(f), float(r) * L)
        if "CONICAL_SURFACE" in names:
            _, f, r, ang = a["CONICAL_SURFACE"]
            return Cone(self.frame(f), float(r) * L, float(ang) * self.angle)
        if "SPHERICAL_SURFACE" in names:
            _, f, r = a["SPHERICAL_SURFACE"]
            return Sphere(self.frame(f), float(r) * L)
        if "DEGENERATE_TOROIDAL_SURFACE" in names:
            _, f, R, r, outer = a["DEGENERATE_TOROIDAL_SURFACE"]
            return Torus(self.frame(f), float(R) * L, float(r) * L, outer == "T")
        if "TOROIDAL_SURFACE" in names:
            _, f, R, r = a["TOROIDAL_SURFACE"]
            return Torus(self.frame(f), float(R) * L, float(r) * L)
        if "SURFACE_OF_REVOLUTION" in names:
            _, c, ax = a["SURFACE_OF_REVOLUTION"]
            an, aa = self.one(ax)
            if an != "AXIS1_PLACEMENT":
                raise Unsupported(f"an axis {an}")
            A = unit(self.direction(aa[2])) if aa[2] is not None else (0.0, 0.0, 1.0)
            return Swept(self.curve(c), revolve=(self.point(aa[1]), A))
        if "OFFSET_SURFACE" in names:
            _, b, d, _ = a["OFFSET_SURFACE"]
            return Offset(self.surface(b), float(d) * L)
        if "SURFACE_OF_LINEAR_EXTRUSION" in names:
            _, c, v = a["SURFACE_OF_LINEAR_EXTRUSION"]
            vn, va = self.one(v)
            return Swept(self.curve(c),
                         extrude=mul(unit(self.direction(va[1])), float(va[2]) * L))
        bs = [n for n in names if n.startswith(("B_SPLINE_SURFACE", "BEZIER_SURFACE",
                                                "UNIFORM_SURFACE", "QUASI_UNIFORM_SURFACE"))]
        if bs:
            return self._bspline_surface(a)
        raise Unsupported(f"a surface {'/'.join(sorted(names))}")

    def _bspline_surface(self, a):
        if len(a) == 1:
            (name, args), = a.items()
            pu, pv, grid = int(args[1]), int(args[2]), args[3]
            if name == "B_SPLINE_SURFACE_WITH_KNOTS":
                U, V = expand(args[8], args[10]), expand(args[9], args[11])
            else:
                U = implicit_knots(name, len(grid) - 1, pu)
                V = implicit_knots(name, len(grid[0]) - 1, pv)
            W = None
        else:
            base = a["B_SPLINE_SURFACE"]
            pu, pv, grid = int(base[0]), int(base[1]), base[2]
            if "B_SPLINE_SURFACE_WITH_KNOTS" in a:
                k = a["B_SPLINE_SURFACE_WITH_KNOTS"]
                U, V = expand(k[0], k[2]), expand(k[1], k[3])
            else:
                kind = next(n for n in a if n in ("BEZIER_SURFACE", "UNIFORM_SURFACE",
                                                  "QUASI_UNIFORM_SURFACE"))
                U = implicit_knots(kind, len(grid) - 1, pu)
                V = implicit_knots(kind, len(grid[0]) - 1, pv)
            W = [[float(w) for w in row] for row in a["RATIONAL_B_SPLINE_SURFACE"][0]] \
                if "RATIONAL_B_SPLINE_SURFACE" in a else None
        pts = [[self.point(q) for q in row] for row in grid]
        return BSplineSurface(pu, pv, pts, W, U, V)


# ---------------------------------------------------------------------------------------------
# a face's loops on its surface's (u, v) plane

class Offset(Surface):
    """A surface moved along its unit normal by d. Its derivatives take the normal's by central
    differences, to about 1e-10."""

    def __init__(self, basis, d):
        self.b, self.d = basis, d
        self.periods, self.domain = basis.periods, basis.domain
        self.breaks, self.angles = basis.breaks, basis.angles
        self.singular = ()

    def _n(self, u, v):
        _, Su, Sv = self.b.ev(u, v)
        return unit(cross(Su, Sv))

    def ev(self, u, v):
        S, Su, Sv = self.b.ev(u, v)
        n = unit(cross(Su, Sv))
        h = 1e-5
        nu = mul(sub(self._n(u + h, v), self._n(u - h, v)), 1 / (2 * h))
        nv = mul(sub(self._n(u, v + h), self._n(u, v - h)), 1 / (2 * h))
        d = self.d
        return add(S, mul(n, d)), add(Su, mul(nu, d)), add(Sv, mul(nv, d))

    def invert(self, q, guess=None):
        if guess is None:
            guess = self.b.invert(q)
        return newton2(self, q, *guess)


class Transposed(Surface):
    """A surface with its parameters swapped, so that the inner integral runs along u."""

    def __init__(self, s):
        self.s = s
        self.periods = s.periods[::-1]
        self.domain = s.domain[::-1]
        self.breaks = s.breaks[::-1]
        self.angles = s.angles[::-1]
        self.nodes = s.nodes[::-1]
        self.singular = tuple((p, 1 - ax, val) for p, ax, val in s.singular)

    def ev(self, u, v):
        S, Su, Sv = self.s.ev(v, u)
        return S, Sv, Su

    def invert(self, q, guess=None):
        u, v = self.s.invert(q, None if guess is None else (guess[1], guess[0]))
        return v, u

    # its normal, S_a × S_b = S_v × S_u, the other way
    def col(self, u, vs):
        return [(x, y, z, -a, -b, -c) for x, y, z, a, b, c in self.s.row(u, vs)]

    def row(self, v, us):
        return [(x, y, z, -a, -b, -c) for x, y, z, a, b, c in self.s.col(v, us)]


def nearest(x, ref, period):
    """`x` moved by whole periods to be nearest `ref`."""
    if not period:
        return x
    return x + round((ref - x) / period) * period


class Path:
    """A piece of a loop on the (u, v) plane: along an edge's curve from t0 to t1, its points
    inverted onto the surface and followed continuously from a reference path, or straight from
    one point to another (`line`), along a singular line."""

    def __init__(self, surface, curve=None, t0=0.0, t1=1.0, line=None):
        self.s, self.c, self.t0, self.t1, self.line = surface, curve, t0, t1, line
        self.ref = []  # [(t, (u, v))]: the path, sampled, followed continuously

    def uv(self, t):
        """(u, v, du/dt, dv/dt) at `t`."""
        if self.line:
            (a, b) = self.line
            du, dv = b[0] - a[0], b[1] - a[1]
            return a[0] + t * du, a[1] + t * dv, du, dv
        q = self.c.point(t)
        g = self.guess(t)
        u, v = self.s.invert(q, g)
        u = nearest(u, g[0], self.s.periods[0])
        v = nearest(v, g[1], self.s.periods[1])
        _, Su, Sv = self.s.ev(u, v)
        d = self.c.deriv(t)
        E, F, G = dot(Su, Su), dot(Su, Sv), dot(Sv, Sv)
        a, b = dot(Su, d), dot(Sv, d)
        det = E * G - F * F
        if det <= 0:
            return u, v, 0.0, 0.0
        return u, v, (G * a - F * b) / det, (E * b - F * a) / det

    def guess(self, t):
        ref = self.ref
        lo, hi = 0, len(ref) - 1
        if (t - ref[0][0]) * (ref[-1][0] - ref[0][0]) <= 0:
            return ref[0][1]
        while hi - lo > 1:
            mid = (lo + hi) // 2
            if (t - ref[mid][0]) * (ref[-1][0] - ref[0][0]) >= 0:
                lo = mid
            else:
                hi = mid
        (ta, a), (tb, b) = ref[lo], ref[hi]
        f = (t - ta) / (tb - ta) if tb != ta else 0.0
        return (a[0] + f * (b[0] - a[0]), a[1] + f * (b[1] - a[1]))

    @property
    def start(self):
        return self.line[0] if self.line else self.ref[0][1]

    @property
    def end(self):
        return self.line[1] if self.line else self.ref[-1][1]

    def shift(self, du, dv):
        if self.line:
            a, b = self.line
            self.line = ((a[0] + du, a[1] + dv), (b[0] + du, b[1] + dv))
        else:
            self.ref = [(t, (p[0] + du, p[1] + dv)) for t, p in self.ref]


def follow(s, curve, t0, t1, scale, seed=None):
    """The edge's curve from t0 to t1 on the surface `s`: its sampled path in (u, v), followed
    continuously, refined where it turns fast; split where it passes through a singular point of
    the surface. [Path], each with its reference path."""
    # where the curve passes through a singular point, it is split
    cuts = []
    for p, axis, val in s.singular:
        ts = [t0 + (t1 - t0) * k / 64 for k in range(65)]
        ds = [norm(sub(curve.point(t), p)) for t in ts]
        for k in range(1, 64):
            if ds[k] <= ds[k - 1] and ds[k] <= ds[k + 1]:
                a, b = ts[k - 1], ts[k + 1]
                for _ in range(80):  # golden section
                    m1 = b - (b - a) * 0.6180339887498949
                    m2 = a + (b - a) * 0.6180339887498949
                    if norm(sub(curve.point(m1), p)) < norm(sub(curve.point(m2), p)):
                        b = m2
                    else:
                        a = m1
                t = (a + b) / 2
                if norm(sub(curve.point(t), p)) < 1e-7 * scale:
                    cuts.append(t)
    cuts = sorted(t for t in cuts if min(abs(t - t0), abs(t - t1)) > 1e-9 * abs(t1 - t0))
    # one singular point may be several lines (a horn torus's centre): one cut there
    cuts = [t for k, t in enumerate(cuts) if not k or t - cuts[k - 1] > 1e-9 * abs(t1 - t0)]
    pieces = []
    bounds = [t0] + cuts + [t1]
    for a, b in zip(bounds, bounds[1:]):
        pieces.append(piece(s, curve, a, b, scale, seed))
        seed = pieces[-1].end
    return pieces


def at_singular(s, q, scale):
    for p, axis, val in s.singular:
        if norm(sub(q, p)) < 1e-7 * scale:
            return axis, val
    return None


def piece(s, curve, t0, t1, scale, seed=None):
    """One piece of an edge, through no singular point but perhaps at its ends."""
    path = Path(s, curve, t0, t1)
    n = 16 + 4 * sum(1 for b in curve.breaks if min(t0, t1) < b < max(t0, t1))
    ts = [t0 + (t1 - t0) * k / n for k in range(n + 1)]
    ends = [at_singular(s, curve.point(t0), scale), at_singular(s, curve.point(t1), scale)]
    # within a hair of a singular end the angles are rounding's: no samples there (the end itself
    # is the limit from inside, below)
    near = [p for p, _, _ in s.singular]

    def clear(t):
        q = curve.point(t)
        return all(norm(sub(q, p)) > 1e-6 * scale for p in near)
    ts = [t for k, t in enumerate(ts) if k in (0, len(ts) - 1) or clear(t)]
    pts = []
    # followed from where the loop's last edge ended (on a surface that comes near itself, the
    # nearest of its samples may be on the wrong fold), and from a point off any singular end (a
    # singular point's angles are rounding's), the ends set below
    prev = seed if ends[0] is None else None
    order = ts[1:] + ts[:1] if ends[0] is not None else ts
    for t in order:
        uv = s.invert(curve.point(t), prev)
        if prev is not None:
            uv = (nearest(uv[0], prev[0], s.periods[0]), nearest(uv[1], prev[1], s.periods[1]))
        pts.append((t, uv))
        prev = uv
    if ends[0] is not None:
        pts = pts[-1:] + pts[:-1]
    # refine where it turns fast (near a pole the angle sweeps): bisect
    k = 0
    while k < len(pts) - 1 and len(pts) < 5000:
        (ta, a), (tb, b) = pts[k], pts[k + 1]
        jump = max(abs(a[0] - b[0]) if s.angles[0] else 0.0,
                   abs(a[1] - b[1]) if s.angles[1] else 0.0)
        tm = (ta + tb) / 2
        if jump > 0.3 and abs(tb - ta) > 1e-12 * abs(t1 - t0) and clear(tm):
            uv = s.invert(curve.point(tm), a)
            uv = (nearest(uv[0], a[0], s.periods[0]), nearest(uv[1], a[1], s.periods[1]))
            pts.insert(k + 1, (tm, uv))
        else:
            k += 1
    # an end at a singular point: the limit along the curve, the singular parameter exact
    for i, end in ((0, ends[0]), (-1, ends[1])):
        if end is None:
            continue
        axis, val = end
        t = pts[i][0]
        h = (t1 - t0) * 1e-5 * (1 if i == 0 else -1)
        g = pts[1][1] if i == 0 else pts[-2][1]
        a = s.invert(curve.point(t + h), g)
        a = (nearest(a[0], g[0], s.periods[0]), nearest(a[1], g[1], s.periods[1]))
        b = s.invert(curve.point(t + 2 * h), a)
        b = (nearest(b[0], a[0], s.periods[0]), nearest(b[1], a[1], s.periods[1]))
        lim = [2 * a[0] - b[0], 2 * a[1] - b[1]]
        lim[axis] = nearest(val, lim[axis], s.periods[axis])
        pts[i] = (t, tuple(lim))
    path.ref = pts
    return path


def closed_edge(m, sv, ev):
    return sv == ev or norm(sub(m.point(sv), m.point(ev))) < 1e-12


def edge_range(m, curve, sv, ev, same_sense):
    """The parameters of the edge from its start vertex to its end vertex on its curve, in the
    direction `same_sense` says."""
    ps, pe = m.point(sv), m.point(ev)
    ts, te = curve.invert(ps), curve.invert(pe)
    if curve.period:
        P = curve.period
        if closed_edge(m, sv, ev):
            te = ts + P if same_sense else ts - P
        elif same_sense:
            te = ts + (te - ts) % P
        else:
            te = ts - (ts - te) % P
    elif closed_edge(m, sv, ev):
        # a closed curve that is not periodic: end to end
        t0, t1 = curve.range
        ts, te = (t0, t1) if same_sense else (t1, t0)
    return ts, te


# ---------------------------------------------------------------------------------------------
# integration

class Face:
    def __init__(self, s, loops, same_sense, centre, scale):
        self.s0, self.loops, self.ss, self.c, self.L = s, loops, same_sense, centre, scale

    def measure(self):
        """The face's contribution to every measure (NM of them), its normal outward where the
        face's own is."""
        loops = self.loops
        wind = [w for _, w in loops]
        # the inner integral runs along the parameter no loop winds round; where none winds, along
        # the one whose ends are poles (a sphere's or a revolution's meridian), else v
        poles = {axis for _, axis, _ in self.s0.singular}
        # free to choose: along the parameter of the surface's poles, else the one of fewer knot
        # spans (each a panel of the inner integral at every point of the outer)
        spans = [len(k) for k in getattr(self.s0, "knots", ((), ()))]
        prefer_u = poles == {0} or (not poles and spans[0] < spans[1])
        if all(w[1] == 0 for w in wind) and not (prefer_u and all(w[0] == 0 for w in wind)):
            s, transposed = self.s0, False
            paths = [p for p, _ in loops]
        elif all(w[0] == 0 for w in wind):
            s, transposed = Transposed(self.s0), True
            paths = []
            for ps, _ in loops:
                for p in ps:
                    p.s = s
                    if p.line:
                        a, b = p.line
                        p.line = ((a[1], a[0]), (b[1], b[0]))
                    else:
                        p.ref = [(t, (uv[1], uv[0])) for t, uv in p.ref]
                paths.append(ps)
        else:
            raise Unsupported("a face whose loops wind round both of its surface's periods")
        self.s = s
        # the inner integral's start: a singular line of the inner parameter, where H is zero, or
        # else where the face is
        # its poles: lines across the inner parameter (and their turns, where it is periodic)
        sing = {val for _, axis, val in s.singular if axis == 1}
        if s.periods[1]:
            sing |= {v + k * s.periods[1] for v in sing for k in range(-2, 3)}
        sing = sorted(sing)
        # a point of the face's boundary, away from any pole
        first = next((p.ref[len(p.ref) // 2][1] for ps in paths for p in ps if not p.line),
                     (0.0, 0.0))
        eps = 1e-12 * (1 + abs(first[1]))
        below = [x for x in sing if x < first[1] - eps]
        above = [x for x in sing if x > first[1] + eps]
        # the inner integral from the pole below the face (H is zero along it), to the pole above
        self.vref = below[-1] if below else above[0] if above else first[1]
        self.top = next((x for x in sing if x > self.vref + eps), None)
        total = [0.0] * NM
        for ps in paths:
            for p in ps:
                part = self.green(p)
                for i in range(NM):
                    total[i] += part[i]
        if not any(ps for ps in paths) and s.periods[1] and not getattr(self.s0, "mirrored", True) \
                and len({round(v % s.periods[1], 9) for _, axis, v in s.singular if axis == 1}) > 1:
            # bounded by points alone, on a surface whose closed profile meets its axis twice: the
            # face could be either part of the profile turned
            raise Unsupported("a face bounded by points alone, on a surface whose closed profile "
                              "meets its axis twice: which part of it is the face is not said")
        sigma = (1 if self.ss else -1) * (-1 if transposed else 1)
        # the loops' sense: by the face's normal (the standard), or by the surface's
        rho = (-1 if transposed else 1) if LOOPS_BY_SURFACE else sigma
        total = [rho * x for x in total]
        # what the loops leave open on a periodic surface: whole turns of the inner integral
        pu = s.periods[0]
        if pu:
            # only a periodic inner parameter, or a singular line across it, leaves a face's
            # loops open: else every side of the face is an edge of it
            if self.top is not None:
                top = self.top
            elif s.periods[1]:
                top = self.vref + s.periods[1]
            else:
                top = None
            span_v = (top - self.vref) if top is not None else None
            if span_v:
                # the whole surface's area in (u, v), signed as its column integral is; the face's
                # must come to (0, |full|]
                full = pu * span_v
                a = abs(full)
                k = -math.floor(total[0] / a - 1e-12) * (1 if full > 0 else -1)
                if k:
                    C = self.column(top)
                    total = [x + k * cx for x, cx in zip(total, C)]
        if total[0] <= 0:
            raise Unsupported("a face whose loops bound no region of its surface")
        # the linear measures by the face's outward normal; its areas are positive
        out = [x * sigma for x in total]
        out[0], out[1] = total[0], total[1]
        return out

    def H(self, u, v):
        """∫ F(u, s) ds from v_ref to v."""
        s, vref = self.s, self.vref
        if abs(v - vref) <= 1e-14 * (1 + abs(vref)):
            return [0.0] * NM
        a, b = (vref, v) if v > vref else (v, vref)
        # breaks: the surface's knots, and its poles (where the normal's length has a kink)
        cuts = [a] + sorted({k for k in s.breaks[1] if a < k < b} | self.poles(a, b)) + [b]
        if s.angles[1]:
            more = [cuts[0]]
            for x, y in zip(cuts, cuts[1:]):
                n = max(1, math.ceil((y - x) / (math.pi / 2)))
                more += [x + (y - x) * k / n for k in range(1, n + 1)]
            cuts = more
        xs, ws = gauss(s.nodes[1])
        nodes, weights = [], []
        for x, y in zip(cuts, cuts[1:]):
            h, m = (y - x) / 2, (y + x) / 2
            nodes += [m + h * xi for xi in xs]
            weights += [h * wi for wi in ws]
        cx, cy, cz = self.c
        a0 = a1 = a2 = a3 = a4 = a5 = a6 = a7 = a8 = a9 = a10 = a11 = 0.0
        for (px, py, pz, nx, ny, nz), w in zip(s.col(u, nodes), weights):
            x, y, z = px - cx, py - cy, pz - cz
            xx, yy, zz = x * x, y * y, z * z
            wx, wy, wz = w * nx, w * ny, w * nz
            a0 += w
            a1 += w * math.sqrt(nx * nx + ny * ny + nz * nz)
            a2 += x * wx + y * wy + z * wz
            a3 += xx * wx
            a4 += yy * wy
            a5 += zz * wz
            a6 += xx * x * wx
            a7 += yy * y * wy
            a8 += zz * z * wz
            a9 += xx * y * wx
            a10 += yy * z * wy
            a11 += zz * x * wz
        out = [a0, a1, a2 / 3, a3 / 2, a4 / 2, a5 / 2, a6 / 3, a7 / 3, a8 / 3, a9 / 2, a10 / 2,
               a11 / 2]
        if v < vref:
            out = [-x for x in out]
        return out

    def poles(self, a, b):
        """The surface's poles strictly between a and b of the inner parameter."""
        if not hasattr(self, "_poles"):
            per = self.s.periods[1]
            vals = {v for _, axis, v in self.s.singular if axis == 1}
            if per:
                vals = {v + k * per for v in vals for k in range(-3, 4)}
            self._poles = sorted(vals)
        return {v for v in self._poles if a + 1e-12 < v < b - 1e-12}

    def column(self, top):
        """∫ H(u, top) du over a whole turn of u: the whole surface, between v_ref and top."""
        pu = self.s.periods[0]
        line = Path(self.s, line=((0.0, top), (pu, top)))
        # -∮ H du along the top, traversed backwards
        return [-x for x in self.green(line)]

    def green(self, path):
        """-∫ H(u(t), v(t)) u'(t) dt along a path."""
        t0, t1 = (0.0, 1.0) if path.line else (path.t0, path.t1)
        cuts = [t0] + sorted(b for b in (path.c.breaks if path.c else ())
                             if min(t0, t1) < b < max(t0, t1)) + [t1]
        if t1 < t0:
            cuts = [t0] + sorted((b for b in cuts[1:-1]), reverse=True) + [t1]
        out = [0.0] * NM
        L = self.L
        tol = [1e-10 * max(L * L, 1.0), 1e-12 * L ** 2, 1e-13 * L ** 3] + [1e-13 * L ** 4] * 3 + \
              [1e-13 * L ** 5] * 6

        spent = [0]

        def f(t):
            spent[0] += 1
            if spent[0] > BUDGET:
                raise Unsupported("the integral round a loop does not converge (a surface that "
                                  "folds or cusps there?)")
            u, v, du, dv = path.uv(t)
            h = self.H(u, v)
            return [-x * du for x in h]

        def panel(a, b, depth):
            hw, mid = (b - a) / 2, (b + a) / 2
            k = [0.0] * NM
            g = [0.0] * NM
            for x, wk, wg in KRONROD:
                y = f(mid + hw * x)
                for i in range(NM):
                    k[i] += wk * y[i]
                    g[i] += wg * y[i]
            if depth < 24 and any(abs(hw * (k[i] - g[i])) > tol[i] + 1e-12 * abs(hw * k[i])
                                  for i in range(NM)):
                l = panel(a, mid, depth + 1)
                r = panel(mid, b, depth + 1)
                return [x + y for x, y in zip(l, r)]
            return [hw * x for x in k]

        # a curve of very many pieces (a polyline, a B-spline fitted point by point): each piece
        # tiny beside the whole, and smooth, so Gauss's seven points are exact to well within the
        # tolerance there, with no estimate of the error
        small = abs(t1 - t0) / 512 if len(cuts) > 513 else 0.0
        gauss = [(x, wg) for x, _, wg in KRONROD if wg]
        for a, b in zip(cuts, cuts[1:]):
            if a == b:
                continue
            if abs(b - a) < small:
                hw, mid = (b - a) / 2, (b + a) / 2
                part = [0.0] * NM
                for x, wg in gauss:
                    y = f(mid + hw * x)
                    for i in range(NM):
                        part[i] += hw * wg * y[i]
            else:
                part = panel(a, b, 0)
            for i in range(NM):
                out[i] += part[i]
        return out


# ---------------------------------------------------------------------------------------------
# the solid

def faces_of(m):
    """Each face of the file's solids: (face entity, its shell's orientation)."""
    out = []
    solids = [r for r, e in m.entities.items()
              if set(e.names) & {"MANIFOLD_SOLID_BREP", "BREP_WITH_VOIDS", "FACETED_BREP"}]
    if not solids:
        raise Unsupported("no solid in the file")
    for r in solids:
        a = m.args(r)
        name = next(n for n in a if n in ("MANIFOLD_SOLID_BREP", "BREP_WITH_VOIDS",
                                          "FACETED_BREP"))
        shells = [(a[name][1], True)]
        if name == "BREP_WITH_VOIDS":
            shells += [(v, True) for v in a[name][2]]
        for sh, sense in shells:
            sn, sa = m.one(sh)
            if sn == "ORIENTED_CLOSED_SHELL":
                sense = sense == (sa[3] != "F")
                sn, sa = m.one(sa[2])
            if sn not in ("CLOSED_SHELL", "OPEN_SHELL"):
                raise Unsupported(f"a shell {sn}")
            for f in sa[1]:
                out.append((f, sense))
    return out, len(solids)


def bounding(m):
    pts = []
    # every point in space: vertices, centres, control points (not the (u, v) points of curves
    # on surfaces, which have two coordinates)
    pts = [m.point(r) for r, e in m.entities.items() if e.name == "CARTESIAN_POINT"
           and len(m.one(r)[1][1]) == 3]
    if not pts:
        raise Unsupported("no points")
    lo = tuple(min(p[i] for p in pts) for i in range(3))
    hi = tuple(max(p[i] for p in pts) for i in range(3))
    return mul(add(lo, hi), 0.5), max(norm(sub(hi, lo)), 1e-9)


def newell(points):
    n = [0.0, 0.0, 0.0]
    for a, b in zip(points, points[1:] + points[:1]):
        n[0] += (a[1] - b[1]) * (a[2] + b[2])
        n[1] += (a[2] - b[2]) * (a[0] + b[0])
        n[2] += (a[0] - b[0]) * (a[1] + b[1])
    return tuple(n)


def poly_paths(s, pts, scale):
    """A polygon's loop on the plane `s`: straight paths between its points."""
    paths = []
    for a, b in zip(pts, pts[1:] + pts[:1]):
        if norm(sub(a, b)) == 0:
            continue
        c = Line(a, sub(b, a))
        paths.append(piece(s, c, 0.0, 1.0, scale))
    return paths


def face_loops(m, f, s, scale):
    """The loops of a face on its surface: [(paths, (winding in u, winding in v))]."""
    name, fa = m.one(f)
    bounds = fa[1]
    loops = []
    for b in bounds:
        bn, ba = m.one(b)
        if bn not in ("FACE_OUTER_BOUND", "FACE_BOUND"):
            raise Unsupported(f"a face bound {bn}")
        loop, orient = ba[1], ba[2] != "F"
        ln, la = m.one(loop)
        if ln == "VERTEX_LOOP":
            continue
        if ln == "POLY_LOOP":
            pts = [m.point(p) for p in la[1]]
            if not orient:
                pts = pts[::-1]
            paths = poly_paths(s, pts, scale)
        elif ln == "EDGE_LOOP":
            paths = []
            last = first_vertex = None
            for oe in la[1]:
                on, oa = m.one(oe)
                if on != "ORIENTED_EDGE":
                    raise Unsupported(f"an edge {on}")
                e, eo = oa[3], oa[4] != "F"
                en, ea = m.one(e)
                if en != "EDGE_CURVE":
                    raise Unsupported(f"an edge {en}")
                sv, ev, geom, same = ea[1], ea[2], ea[3], ea[4] != "F"
                ends = (sv, ev) if eo else (ev, sv)  # as the loop runs, before its bound turns it
                if last is not None and norm(sub(m.point(last), m.point(ends[0]))) > 1e-6 * scale:
                    raise Invalid("a loop whose edges do not meet end to start")
                last = ends[1]
                first_vertex = ends[0] if first_vertex is None else first_vertex
                curve = m.curve(geom)
                if isinstance(curve, Line) and (not curve.dd or closed_edge(m, sv, ev)):
                    continue  # an edge with no extent: a point
                ts, te = edge_range(m, curve, sv, ev, same)
                if eo != orient:
                    ts, te = te, ts
                pieces = follow(s, curve, ts, te, scale, paths[-1].end if paths else None)
                paths += pieces
            if last is not None and norm(sub(m.point(last), m.point(first_vertex))) > 1e-6 * scale:
                raise Invalid("a loop whose edges do not close")
            if not paths:
                continue  # a loop of points only
            if not orient:
                pass  # already reversed edge by edge; the order of edges is reversed below
            if not orient:
                paths = paths[::-1]
        else:
            raise Unsupported(f"a loop {ln}")
        loops.append(join(s, paths, scale))
    return loops


def join(s, paths, scale):
    """A loop's paths joined end to start on the (u, v) plane: each moved by whole periods to
    meet the last, a path along a singular line inserted where the loop passes through a pole;
    and how many times the loop winds round each period. (paths, (wu, wv))."""
    if not paths:
        raise Unsupported("an empty loop")
    out = [paths[0]]
    for p in paths[1:]:
        prev = out[-1].end
        st = p.start
        du = nearest(st[0], prev[0], s.periods[0]) - st[0]
        dv = nearest(st[1], prev[1], s.periods[1]) - st[1]
        p.shift(du, dv)
        gap(s, out, prev, p.start, scale)
        out.append(p)
    # closing: the start, moved by whole periods to be nearest the end
    end, st = out[-1].end, out[0].start
    nu = round((end[0] - st[0]) / s.periods[0]) if s.periods[0] else 0
    nv = round((end[1] - st[1]) / s.periods[1]) if s.periods[1] else 0
    target = (st[0] + nu * (s.periods[0] or 0), st[1] + nv * (s.periods[1] or 0))
    gap(s, out, end, target, scale)
    return out, (nu, nv)


def gap(s, out, a, b, scale):
    """Join `a` to `b`: nothing if they meet, else a straight path along a singular line."""
    du, dv = b[0] - a[0], b[1] - a[1]
    if abs(du) < 1e-7 and abs(dv) < 1e-7:
        return
    S1 = s.ev(*a)[0]
    for p, axis, val in s.singular:
        if norm(sub(S1, p)) < 1e-6 * scale:
            out.append(Path(s, line=(a, b)))
            return
    if norm(sub(S1, s.ev(*b)[0])) < 1e-5 * scale:
        # the same point to within a writer's tolerance (an edge's curve ending a hair from its
        # vertex), or apart in (u, v) where the surface's parameters run off (a closed B-spline
        # not quite periodic): joined straight, which moves the measures by about the gap
        out.append(Path(s, line=(a, b)))
        return
    raise Unsupported(f"a loop that does not join up on its surface ({a} to {b})")


def measure(text):
    """The mass properties of the solid in a STEP file's text: {volume, area, centroid, inertia
    (about the centroid), solids, faces}, lengths in millimetres."""
    m = Model(text)
    faces, nsolids = faces_of(m)
    centre, scale = bounding(m)
    total = [0.0] * NM
    for f, sense in faces:
        fn, fa = m.one(f)
        if fn in ("ADVANCED_FACE", "FACE_SURFACE"):
            s = m.surface(fa[2])
            same = fa[3] != "F"
        elif fn == "FACE":
            # a faceted B-rep's polygon: on the plane its points span
            bounds = fa[1]
            ln, la = m.one(m.one(bounds[0])[1][1])
            pts = [m.point(p) for p in la[1]]
            Z = unit(newell(pts))
            X = unit(sub(pts[1], pts[0]))
            X = unit(sub(X, mul(Z, dot(X, Z))))
            s = Plane((pts[0], X, cross(Z, X), Z))
            same = True
        else:
            raise Unsupported(f"a face {fn}")
        try:
            loops = face_loops(m, f, s, scale)
            part = Face(s, loops, same, centre, scale).measure()
        except Unsupported as e:
            raise type(e)(f"face #{f}: {e}") from None
        sign = 1 if sense else -1
        total[1] += part[1]
        for i in range(2, NM):
            total[i] += sign * part[i]
    V = total[2]
    if not V:
        raise Unsupported("the solid has no volume")
    g = (total[3] / V, total[4] / V, total[5] / V)
    sxx = total[6] - V * g[0] * g[0]
    syy = total[7] - V * g[1] * g[1]
    szz = total[8] - V * g[2] * g[2]
    sxy = total[9] - V * g[0] * g[1]
    syz = total[10] - V * g[1] * g[2]
    szx = total[11] - V * g[2] * g[0]
    inertia = [[syy + szz, -sxy, -szx], [-sxy, sxx + szz, -syz], [-szx, -syz, sxx + syy]]
    return {"volume": V, "area": total[1], "centroid": list(add(centre, g)),
            "inertia": inertia, "solids": nsolids, "faces": len(faces)}


def read(path):
    """`measure` of the STEP file at `path`, or {"invalid": why} where the file holds no solid
    (whoever reads it), or {"error": why} where the measurer cannot measure it."""
    try:
        with open(path, encoding="latin-1") as f:
            return measure(f.read())
    except Invalid as e:
        return {"invalid": str(e)}
    except (Unsupported, ValueError, ZeroDivisionError, IndexError, KeyError, TypeError) as e:
        return {"error": f"{type(e).__name__ if not isinstance(e, Unsupported) else 'cannot measure'}"
                         f": {e}"}


if __name__ == "__main__":
    args = sys.argv[1:]
    if args and args[0] == "--loops-by-surface":
        LOOPS_BY_SURFACE = True
        args = args[1:]
    for p in args:
        print(json.dumps(read(p)))
