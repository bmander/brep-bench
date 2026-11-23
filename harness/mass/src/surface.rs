//! Surfaces: ev(u, v) -> (S, S_u, S_v); invert(p, guess) -> (u, v); `periods` (pu, pv), each none
//! or the period; `domain` ((u0, u1), (v0, v1)), infinite where unbounded; `singular`: (point,
//! axis 0 or 1, value), the lines u = value or v = value where the surface collapses to a point;
//! `breaks` (u knots, v knots); `angles`: the parameters that are angles, whose quadrature is
//! split into panels of no more than a quarter turn.
use crate::curve::*;
use crate::geom::*;
use crate::py::*;
use std::cell::RefCell;
use std::sync::{Arc, OnceLock, RwLock};

const INF: f64 = f64::INFINITY;
pub type Uv = (f64, f64);

#[derive(Clone, Copy)]
pub struct Sing {
    pub p: V3,
    pub axis: usize,
    pub val: f64,
}

pub struct Surface {
    pub periods: [Option<f64>; 2],
    pub domain: [(f64, f64); 2],
    pub singular: RwLock<Vec<Sing>>,
    pub breaks: [Vec<f64>; 2],
    pub angles: [bool; 2],
    /// The Gauss points for each panel of the inner integral, along each parameter: 16 for an
    /// angle (a panel no more than a quarter turn) or a rational piece, fewer where it is a
    /// polynomial.
    pub nodes: [usize; 2],
    pub kind: Kind,
}

pub enum Kind {
    Plane(Frame),
    /// A surface of a frame's revolution, O + ρ(v)(cos u X + sin u Y) + h(v) Z, u an angle.
    Cylinder(Frame, f64),
    Cone(Frame, f64, f64),
    Sphere(Frame, f64),
    Torus(Frame, f64, f64, Option<bool>),
    BSpline(BSplineSurface),
    Swept(Swept),
    Offset(Arc<Surface>, f64),
}

pub struct BSplineSurface {
    pu: usize,
    pv: usize,
    /// The control points, a row for each along u.
    pts: Vec<V3>,
    w: Option<Vec<f64>>,
    u: Vec<f64>,
    v: Vec<f64>,
    nu: usize,
    nv: usize,
    domain: [(f64, f64); 2],
    periods: [Option<f64>; 2],
    pub knots: [Vec<f64>; 2],
    size: f64,
    samples: OnceLock<Vec<(Uv, V3)>>,
}

/// A surface of revolution, σ(u, v) = C + (λ(v) - C) turned by the angle u about the axis
/// (ISO 10303-42's: u the angle, v the swept curve's parameter), or of linear extrusion,
/// σ(u, v) = λ(u) + v V (u the curve's, v along V).
pub struct Swept {
    c: Arc<Curve>,
    revolve: bool,
    a0: V3,
    a: V3,
    x: V3,
    y: V3,
    pub mirrored: bool,
    d: V3,
    samples: OnceLock<Vec<f64>>,
}

/// What a column or a row of a B-spline surface is worked out in, kept from one to the next.
#[derive(Default)]
struct Scratch {
    js: Vec<usize>,
    nb: Vec<f64>,
    db: Vec<f64>,
    q: Vec<[f64; 4]>,
    r: Vec<[f64; 4]>,
}

thread_local! {
    static SCRATCH: RefCell<Scratch> = RefCell::new(Scratch::default());
}

/// `max(xs)`.
fn maximum(xs: impl IntoIterator<Item = f64>) -> f64 {
    let mut it = xs.into_iter();
    let first = it.next().unwrap();
    it.fold(first, pymax)
}

fn minimum(xs: impl IntoIterator<Item = f64>) -> f64 {
    let mut it = xs.into_iter();
    let first = it.next().unwrap();
    it.fold(first, pymin)
}

#[inline(always)]
fn normal(su: V3, sv: V3) -> V3 {
    [
        su[1] * sv[2] - su[2] * sv[1],
        su[2] * sv[0] - su[0] * sv[2],
        su[0] * sv[1] - su[1] * sv[0],
    ]
}

impl BSplineSurface {
    #[allow(clippy::too_many_arguments)]
    fn new(
        pu: i64,
        pv: i64,
        grid: Vec<Vec<V3>>,
        weights: Option<Vec<Vec<f64>>>,
        u: Vec<f64>,
        v: Vec<f64>,
    ) -> R<(BSplineSurface, [usize; 2], Vec<Sing>)> {
        let nu = grid.len() as i64 - 1;
        let nv = match grid.first() {
            Some(row) => row.len() as i64 - 1,
            None => return index_error(),
        };
        if u.len() as i64 != nu + pu + 2 || v.len() as i64 != nv + pv + 2 {
            return unsupported("a B-spline surface whose knots do not match its control points");
        }
        if pu < 0 || pv < 0 || nu < pu || nv < pv || grid.iter().any(|r| (r.len() as i64) < nv + 1)
        {
            // (Python reads its lists from their ends there, or past them)
            return index_error();
        }
        if let Some(w) = &weights {
            if w.len() < grid.len() || w.iter().any(|r| (r.len() as i64) < nv + 1) {
                return index_error();
            }
        }
        let (pu, pv, nu, nv) = (pu as usize, pv as usize, nu as usize, nv as usize);
        let domain = [(u[pu], u[nu + 1]), (v[pv], v[nv + 1])];
        let ku = sorted_set(u[pu..nu + 2].iter().copied());
        let kv = sorted_set(v[pv..nv + 2].iter().copied());
        let all: Vec<V3> = grid.iter().flatten().copied().collect();
        let lo: Vec<f64> = (0..3).map(|i| minimum(all.iter().map(|q| q[i]))).collect();
        let hi: Vec<f64> = (0..3).map(|i| maximum(all.iter().map(|q| q[i]))).collect();
        let pts: Vec<V3> = grid
            .iter()
            .flat_map(|r| r[..nv + 1].iter().copied())
            .collect();
        let mut size = norm(sub([hi[0], hi[1], hi[2]], [lo[0], lo[1], lo[2]]));
        if size == 0.0 {
            size = 1.0;
        }
        let eps = 1e-9 * size;
        // a polynomial along a parameter where the weights do not vary along it
        let flat = |axis: usize| -> bool {
            let Some(w) = &weights else { return true };
            let level = |r: &mut dyn Iterator<Item = f64>| {
                let r: Vec<f64> = r.collect();
                let (mx, mn) = (maximum(r.iter().copied()), minimum(r.iter().copied()));
                mx - mn <= 1e-12 * mx
            };
            if axis == 1 {
                w.iter().all(|r| level(&mut r.iter().copied()))
            } else {
                let cols = w.iter().map(|r| r.len()).min().unwrap_or(0);
                (0..cols).all(|j| level(&mut w.iter().map(|r| r[j])))
            }
        };
        let nodes = [
            if flat(0) { curved(pu) } else { 16 },
            if flat(1) { curved(pv) } else { 16 },
        ];
        let w = weights.map(|w| w.iter().flat_map(|r| r[..nv + 1].iter().copied()).collect());
        let mut s = BSplineSurface {
            pu,
            pv,
            pts,
            w,
            u,
            v,
            nu,
            nv,
            domain,
            periods: [None, None],
            knots: [ku, kv],
            size,
            samples: OnceLock::new(),
        };
        // closed where opposite boundaries meet; periodic for the unwrapping of loops
        let at = |s: &BSplineSurface, axis: usize, a: f64, b: f64| -> R<V3> {
            Ok(if axis == 0 {
                s.ev_raw(a, b)?.0
            } else {
                s.ev_raw(b, a)?.0
            })
        };
        let mut per = [None, None];
        for axis in 0..2 {
            let ((a0, a1), (b0, b1)) = (domain[axis], domain[1 - axis]);
            let mut ok = true;
            for k in 0..9 {
                let b = b0 + (b1 - b0) * k as f64 / 8.0;
                let pa = at(&s, axis, a0, b)?;
                let pb = at(&s, axis, a1, b)?;
                if norm(sub(pa, pb)) > eps {
                    ok = false;
                    break;
                }
            }
            if ok && a1 - a0 != 0.0 {
                per[axis] = Some(a1 - a0);
            }
        }
        s.periods = per;
        // collapsed boundaries: a pole
        let mut sing = Vec::new();
        for axis in 0..2 {
            for end in [domain[axis].0, domain[axis].1] {
                let (b0, b1) = domain[1 - axis];
                let mut ps = Vec::with_capacity(9);
                for k in 0..9 {
                    ps.push(at(&s, axis, end, b0 + (b1 - b0) * k as f64 / 8.0)?);
                }
                if maximum(ps.iter().map(|q| norm(sub(*q, ps[0])))) < eps {
                    sing.push(Sing {
                        p: ps[0],
                        axis,
                        val: end,
                    });
                }
            }
        }
        Ok((s, nodes, sing))
    }

    fn wrap(&self, u: f64, v: f64) -> R<Uv> {
        let u = match self.periods[0] {
            Some(per) => self.domain[0].0 + pymod(u - self.domain[0].0, per)?,
            None => u,
        };
        let v = match self.periods[1] {
            Some(per) => self.domain[1].0 + pymod(v - self.domain[1].0, per)?,
            None => v,
        };
        Ok((u, v))
    }

    /// (S, S_u, S_v) at (u, v), as they are given (not brought within a period).
    fn ev_raw(&self, u: f64, v: f64) -> R<(V3, V3, V3)> {
        let (pu, pv) = (self.pu, self.pv);
        let i = span(self.nu, pu, u, &self.u);
        let j = span(self.nv, pv, v, &self.v);
        let mut stack = [0.0f64; 4 * (DEG + 1)];
        let mut heap;
        let work: &mut [f64] = if pu <= DEG && pv <= DEG {
            &mut stack
        } else {
            heap = vec![0.0f64; 2 * (pu + 1) + 2 * (pv + 1)];
            &mut heap
        };
        let (nu_, rest) = work.split_at_mut(pu + 1);
        let (du_, rest) = rest.split_at_mut(pu + 1);
        let (nv_, rest) = rest.split_at_mut(pv + 1);
        let dv_ = &mut rest[..pv + 1];
        basis(i, u, pu, &self.u, nu_, du_);
        basis(j, v, pv, &self.v, nv_, dv_);
        let cols = self.nv + 1;
        let mut s = [0.0f64; 4];
        let mut su = [0.0f64; 4];
        let mut sv = [0.0f64; 4];
        for a in 0..=pu {
            let row = (i - pu + a) * cols + (j - pv);
            let (na, da) = (nu_[a], du_[a]);
            for b in 0..=pv {
                let q = self.pts[row + b];
                let w = match &self.w {
                    Some(w) => w[row + b],
                    None => 1.0,
                };
                let (nb, db) = (nv_[b], dv_[b]);
                let (f, fu, fv) = (na * nb * w, da * nb * w, na * db * w);
                s[0] += f * q[0];
                s[1] += f * q[1];
                s[2] += f * q[2];
                s[3] += f;
                su[0] += fu * q[0];
                su[1] += fu * q[1];
                su[2] += fu * q[2];
                su[3] += fu;
                sv[0] += fv * q[0];
                sv[1] += fv * q[1];
                sv[2] += fv * q[2];
                sv[3] += fv;
            }
        }
        let w = s[3];
        if w == 0.0 {
            return zero_division();
        }
        let p = [s[0] / w, s[1] / w, s[2] / w];
        let du = [
            (su[0] - su[3] * p[0]) / w,
            (su[1] - su[3] * p[1]) / w,
            (su[2] - su[3] * p[2]) / w,
        ];
        let dv = [
            (sv[0] - sv[3] * p[0]) / w,
            (sv[1] - sv[3] * p[1]) / w,
            (sv[2] - sv[3] * p[2]) / w,
        ];
        Ok((p, du, dv))
    }

    // along a column (u fixed) the u basis is the same at every point: the net is collapsed once to
    // a curve in v and its derivative in u, homogeneous, and each point costs a curve's sum
    /// (x, y, z, nx, ny, nz) at parameter `t` of `axis` fixed, each s of the other.
    fn line(&self, axis: usize, t: f64, ss: &[f64], out: &mut Vec<[f64; 6]>) -> R<()> {
        out.clear();
        let a0 = self.domain[axis].0;
        let t = match self.periods[axis] {
            Some(per) => a0 + pymod(t - a0, per)?,
            None => t,
        };
        let (p, n, kk) = if axis == 0 {
            (self.pu, self.nu, &self.u)
        } else {
            (self.pv, self.nv, &self.v)
        };
        let (q, m, ll) = if axis == 0 {
            (self.pv, self.nv, &self.v)
        } else {
            (self.pu, self.nu, &self.u)
        };
        let i = span(n, p, t, kk);
        let (mut sn, mut sd) = ([0.0; DEG + 1], [0.0; DEG + 1]);
        let (mut hn, mut hd);
        let (nn, dd): (&mut [f64], &mut [f64]) = if p <= DEG {
            (&mut sn[..p + 1], &mut sd[..p + 1])
        } else {
            hn = vec![0.0; p + 1];
            hd = vec![0.0; p + 1];
            (&mut hn, &mut hd)
        };
        basis(i, t, p, kk, nn, dd);
        if ss.is_empty() {
            return Ok(());
        }
        let b0 = self.domain[1 - axis].0;
        let per2 = self.periods[1 - axis];
        let cols = self.nv + 1;
        SCRATCH.with(|scratch| {
            let mut scratch = scratch.borrow_mut();
            let Scratch {
                js,
                nb,
                db,
                q: qs,
                r: rs,
            } = &mut *scratch;
            // the basis at each point first, so that only the control points they touch are
            // collapsed
            js.clear();
            nb.resize(ss.len() * (q + 1), 0.0);
            db.resize(ss.len() * (q + 1), 0.0);
            let (mut lo, mut hi) = (usize::MAX, 0);
            for (k, &s) in ss.iter().enumerate() {
                let s = match per2 {
                    Some(per) => b0 + pymod(s - b0, per)?,
                    None => s,
                };
                let j = span(m, q, s, ll);
                let at = k * (q + 1);
                basis(
                    j,
                    s,
                    q,
                    ll,
                    &mut nb[at..at + q + 1],
                    &mut db[at..at + q + 1],
                );
                js.push(j);
                lo = lo.min(j - q);
                hi = hi.max(j);
            }
            qs.clear();
            rs.clear();
            for j in lo..=hi {
                let (mut qq, mut rr) = ([0.0f64; 4], [0.0f64; 4]);
                for a in 0..=p {
                    let ii = i - p + a;
                    let at = if axis == 0 {
                        ii * cols + j
                    } else {
                        j * cols + ii
                    };
                    let pt = self.pts[at];
                    let w = match &self.w {
                        Some(w) => w[at],
                        None => 1.0,
                    };
                    let (nw, dw) = (nn[a] * w, dd[a] * w);
                    qq[0] += nw * pt[0];
                    qq[1] += nw * pt[1];
                    qq[2] += nw * pt[2];
                    qq[3] += nw;
                    rr[0] += dw * pt[0];
                    rr[1] += dw * pt[1];
                    rr[2] += dw * pt[2];
                    rr[3] += dw;
                }
                qs.push(qq);
                rs.push(rr);
            }
            for (k, &j) in js.iter().enumerate() {
                let at = k * (q + 1);
                let (mut s, mut t, mut u) = ([0.0f64; 4], [0.0f64; 4], [0.0f64; 4]);
                for b in 0..=q {
                    let (qq, rr) = (&qs[j - q + b - lo], &rs[j - q + b - lo]);
                    let (nb, db) = (nb[at + b], db[at + b]);
                    s[0] += nb * qq[0];
                    s[1] += nb * qq[1];
                    s[2] += nb * qq[2];
                    s[3] += nb * qq[3];
                    t[0] += nb * rr[0]; // along the fixed axis
                    t[1] += nb * rr[1];
                    t[2] += nb * rr[2];
                    t[3] += nb * rr[3];
                    u[0] += db * qq[0]; // along the free axis
                    u[1] += db * qq[1];
                    u[2] += db * qq[2];
                    u[3] += db * qq[3];
                }
                let sw = s[3];
                if sw == 0.0 {
                    return zero_division();
                }
                let (x, y, z) = (s[0] / sw, s[1] / sw, s[2] / sw);
                let (ax, ay, az) = (
                    (t[0] - t[3] * x) / sw,
                    (t[1] - t[3] * y) / sw,
                    (t[2] - t[3] * z) / sw,
                );
                let (bx, by, bz) = (
                    (u[0] - u[3] * x) / sw,
                    (u[1] - u[3] * y) / sw,
                    (u[2] - u[3] * z) / sw,
                );
                if axis == 0 {
                    // S_u = a, S_v = b
                    out.push([
                        x,
                        y,
                        z,
                        ay * bz - az * by,
                        az * bx - ax * bz,
                        ax * by - ay * bx,
                    ]);
                } else {
                    // S_u = b, S_v = a
                    out.push([
                        x,
                        y,
                        z,
                        by * az - bz * ay,
                        bz * ax - bx * az,
                        bx * ay - by * ax,
                    ]);
                }
            }
            Ok(())
        })
    }

    /// Points of the surface for a first guess at inversion, ((u, v), point): four a knot span
    /// each way, a column at a time, made when first wanted.
    fn samples(&self) -> R<&Vec<(Uv, V3)>> {
        if let Some(s) = self.samples.get() {
            return Ok(s);
        }
        let along = |k: &Vec<f64>| -> R<Vec<f64>> {
            let mut out = Vec::new();
            for w in k.windows(2) {
                for i in 0..4 {
                    out.push(w[0] + (w[1] - w[0]) * i as f64 / 4.0);
                }
            }
            match k.last() {
                Some(x) => out.push(*x),
                None => return index_error(),
            }
            Ok(out)
        };
        let mut su = along(&self.knots[0])?;
        let mut sv = along(&self.knots[1])?;
        while su.len() * sv.len() > 40000 {
            if su.len() > sv.len() {
                su = su.into_iter().step_by(2).collect();
            } else {
                sv = sv.into_iter().step_by(2).collect();
            }
        }
        let mut out = Vec::with_capacity(su.len() * sv.len());
        let mut col = Vec::new();
        for &u in &su {
            self.line(0, u, &sv, &mut col)?;
            for (v, c) in sv.iter().zip(&col) {
                out.push(((u, *v), [c[0], c[1], c[2]]));
            }
        }
        Ok(self.samples.get_or_init(|| out))
    }

    fn nearest_sample(&self, q: V3) -> R<Uv> {
        let mut best: Option<(f64, Uv)> = None;
        for (uv, p) in self.samples()? {
            let e = sub(*p, q);
            let d = dot(e, e);
            if best.is_none() || d < best.unwrap().0 {
                best = Some((d, *uv));
            }
        }
        match best {
            Some(b) => Ok(b.1),
            None => Err(Err::Py(
                "ValueError",
                "min() iterable argument is empty".into(),
            )),
        }
    }

    fn off(&self, uv: Uv, q: V3) -> R<f64> {
        let (u, v) = self.wrap(uv.0, uv.1)?;
        Ok(norm(sub(self.ev_raw(u, v)?.0, q)))
    }

    fn invert(&self, me: &Surface, q: V3, guess: Option<Uv>) -> R<Uv> {
        let Some(guess) = guess else {
            // from each of the nearest few samples, until one finds the point on the surface (on a
            // surface that winds near itself, the nearest may lead Newton to another turn of it)
            let samples = self.samples()?;
            let mut near: Vec<(f64, usize)> = Vec::with_capacity(7);
            for (k, (_, p)) in samples.iter().enumerate() {
                let e = sub(*p, q);
                let d = dot(e, e);
                if near.len() < 6 || d < near[5].0 {
                    let at = near.iter().position(|e| d < e.0).unwrap_or(near.len());
                    near.insert(at, (d, k));
                    near.truncate(6);
                }
            }
            let mut best: Option<(f64, Uv)> = None;
            for (_, k) in near {
                let (u, v) = samples[k].0;
                let uv = newton2(me, q, u, v)?;
                let off = self.off(uv, q)?;
                if best.is_none() || off < best.unwrap().0 {
                    best = Some((off, uv));
                }
                if off < 1e-9 * self.size {
                    break;
                }
            }
            return match best {
                Some(b) => Ok(b.1),
                None => Err(Err::Py(
                    "TypeError",
                    "'NoneType' object is not subscriptable".into(),
                )),
            };
        };
        let (u, v) = self.wrap(guess.0, guess.1)?;
        let uv = newton2(me, q, u, v)?;
        if self.off(uv, q)? > 1e-4 * self.size {
            // Newton from the guess found another fold of the surface: from the nearest sample
            let (u, v) = self.nearest_sample(q)?;
            let other = newton2(me, q, u, v)?;
            if self.off(other, q)? < self.off(uv, q)? {
                return Ok(other);
            }
        }
        Ok(uv)
    }
}

/// The (u, v) of the point of `s` nearest `q`, by Newton's method from (u, v), kept within
/// the surface's bounded parameters.
///
/// Where the surface's two rates run together (along a fold of an envelope, where S_u × S_v
/// passes through nothing though neither rate does) Newton's step is the quotient of two
/// roundings: it never settles, and its last point may be far from `q`. A Newton that ran out
/// of steps still asking for one is tried again with its steps damped (`damped`), and the
/// damped point is the answer where it is nearer `q` by more than rounding. One that settled
/// answers as it always has, to the last bit.
fn newton2(s: &Surface, q: V3, mut u: f64, mut v: f64) -> R<Uv> {
    let [(u0, u1), (v0, v1)] = s.domain;
    let from = (u, v);
    // whether it stopped moving, and the last step it asked for
    let (mut settled, mut asked) = (false, 0.0);
    for _ in 0..60 {
        let (p, su, sv) = s.ev(u, v)?;
        let e = sub(q, p);
        let (ee, ff, gg) = (dot(su, su), dot(su, sv), dot(sv, sv));
        let (a, b) = (dot(su, e), dot(sv, e));
        let det = ee * gg - ff * ff;
        let (mut du, mut dv);
        if det <= 1e-300 {
            // at a pole: step along the parameter that moves the point
            if ee > gg && ee != 0.0 {
                (du, dv) = (a / ee, 0.0);
            } else if gg != 0.0 {
                (du, dv) = (0.0, b / gg);
            } else {
                settled = true;
                break;
            }
        } else {
            (du, dv) = ((gg * a - ff * b) / det, (ee * b - ff * a) / det);
        }
        // a step no longer than a quarter of the parameter's span (near a pole the metric
        // vanishes, and a full step would leave the surface's domain by orders of magnitude)
        for (i, (a, b)) in [(u0, u1), (v0, v1)].into_iter().enumerate() {
            let d = if i == 0 { &mut du } else { &mut dv };
            let span = s.periods[i].unwrap_or(b - a);
            if span.is_finite() && d.abs() > span / 4.0 {
                *d = (span / 4.0).copysign(*d);
            }
        }
        asked = pymax(du.abs() / (1.0 + u.abs()), dv.abs() / (1.0 + v.abs()));
        let (mut nu, mut nv) = (u + du, v + dv);
        match s.periods[0] {
            None => nu = pymin(pymax(nu, u0), u1),
            Some(per) if u0.is_finite() => nu = u0 + pymod(nu - u0, per)?,
            _ => {}
        }
        match s.periods[1] {
            None => nv = pymin(pymax(nv, v0), v1),
            Some(per) if v0.is_finite() => nv = v0 + pymod(nv - v0, per)?,
            _ => {}
        }
        let done =
            (nu - u).abs() < 1e-15 * (1.0 + u.abs()) && (nv - v).abs() < 1e-15 * (1.0 + v.abs());
        (u, v) = (nu, nv);
        if done {
            settled = true;
            break;
        }
    }
    if !settled && asked > UNSETTLED {
        let off = norm(sub(q, s.ev(u, v)?.0));
        let (uv, nearer) = damped(s, q, from.0, from.1)?;
        if nearer < off - UNSETTLED * (1.0 + pymax(pymax(q[0].abs(), q[1].abs()), q[2].abs())) {
            return Ok(uv);
        }
    }
    Ok((u, v))
}

/// The step, as a fraction of its parameter's size, that a Newton out of steps may still ask
/// for and be rounding's alone; and how much nearer the point, as a fraction of its place's
/// size, the damped iteration must come to be taken instead.
const UNSETTLED: f64 = 1e-9;

/// The (u, v) of a point of `s` near `q`, and how far from `q` it is: from (u, v) by Newton's
/// steps damped (Levenberg's), each taken only if it comes nearer, so that where the surface's
/// rates run together and the undamped step is noise it still comes down onto the surface.
/// Along such a fold the nearest point is not one the arithmetic can tell from its neighbours
/// along the fold: this is a point as near `q` as they are, which is what a loop laid through
/// it needs.
fn damped(s: &Surface, q: V3, mut u: f64, mut v: f64) -> R<(Uv, f64)> {
    let [(u0, u1), (v0, v1)] = s.domain;
    let held = |x: f64, i: usize| -> R<f64> {
        let (a, b) = [(u0, u1), (v0, v1)][i];
        match s.periods[i] {
            None => Ok(pymin(pymax(x, a), b)),
            Some(per) if a.is_finite() => Ok(a + pymod(x - a, per)?),
            _ => Ok(x),
        }
    };
    (u, v) = (held(u, 0)?, held(v, 1)?);
    let (p, mut su, mut sv) = s.ev(u, v)?;
    let mut e = sub(q, p);
    let mut d2 = dot(e, e);
    let mut mu = 1e-3;
    for _ in 0..200 {
        let (ee, ff, gg) = (dot(su, su), dot(su, sv), dot(sv, sv));
        let (a, b) = (dot(su, e), dot(sv, e));
        let most = pymax(ee, gg);
        if most == 0.0 {
            break;
        }
        // the damping raised until the step comes nearer
        let mut moved = None;
        while mu <= 1e12 {
            let (m, n) = (ee + mu * most, gg + mu * most);
            let det = m * n - ff * ff;
            let (nu, nv) = (
                held(u + (n * a - ff * b) / det, 0)?,
                held(v + (m * b - ff * a) / det, 1)?,
            );
            let at = s.ev(nu, nv)?;
            let ne = sub(q, at.0);
            if dot(ne, ne) < d2 {
                moved = Some((nu, nv, at, ne));
                break;
            }
            mu *= 10.0;
        }
        let Some((nu, nv, at, ne)) = moved else { break };
        let done =
            (nu - u).abs() < 1e-15 * (1.0 + u.abs()) && (nv - v).abs() < 1e-15 * (1.0 + v.abs());
        (u, v) = (nu, nv);
        (su, sv) = (at.1, at.2);
        e = ne;
        d2 = dot(e, e);
        mu = pymax(mu / 10.0, 1e-15);
        if done {
            break;
        }
    }
    Ok(((u, v), d2.sqrt()))
}

impl Swept {
    fn rot(&self, w: V3, a: f64) -> R<V3> {
        let (c, s) = (cosine(a)?, sine(a)?);
        Ok(lin3(
            c,
            w,
            s,
            cross(self.a, w),
            (1.0 - c) * dot(self.a, w),
            self.a,
        ))
    }

    /// Where the profile meets the axis: the poles, lines v = t of the surface.
    fn on_axis(&self) -> R<Vec<Sing>> {
        let c = &*self.c;
        let (mut t0, mut t1) = c.range();
        if let Curve::Line { p, d, .. } = c {
            // a line meets the axis at most once: where it comes nearest
            let w0 = sub(*p, self.a0);
            let dd = sub(*d, mul(self.a, dot(*d, self.a)));
            let ww = sub(w0, mul(self.a, dot(w0, self.a)));
            if dot(dd, dd) == 0.0 {
                return Ok(Vec::new());
            }
            let t = -dot(ww, dd) / dot(dd, dd);
            if norm(add(ww, mul(dd, t))) < 1e-9 * (1.0 + norm(w0)) {
                return Ok(vec![Sing {
                    p: c.point(t)?,
                    axis: 1,
                    val: t,
                }]);
            }
            return Ok(Vec::new());
        }
        if !(t0.is_finite() && t1.is_finite()) {
            // a parabola's or a hyperbola's parameter: its turns near the vertex are what meet
            // the axis
            (t0, t1) = (-100.0, 100.0);
        }
        let r = |t: f64| -> R<f64> {
            let w = sub(c.point(t)?, self.a0);
            Ok(norm(sub(w, mul(self.a, dot(w, self.a)))))
        };
        let mut cuts = vec![t0, t1];
        cuts.extend(c.breaks().iter().copied().filter(|b| t0 < *b && *b < t1));
        let cuts = sorted_set(cuts);
        let n = if c.range().0.is_finite() { 16 } else { 4000 };
        let mut ts = Vec::new();
        for w in cuts.windows(2) {
            for k in 0..n {
                ts.push(w[0] + (w[1] - w[0]) * k as f64 / n as f64);
            }
        }
        ts.push(t1);
        let mut size = f64::NEG_INFINITY;
        for (k, t) in ts.iter().enumerate() {
            let d = norm(sub(c.point(*t)?, self.a0));
            size = if k == 0 { d } else { pymax(size, d) };
        }
        if size == 0.0 {
            size = 1.0;
        }
        let mut rs = Vec::with_capacity(ts.len());
        for t in &ts {
            rs.push(r(*t)?);
        }
        let mut found: Vec<Sing> = Vec::new();
        for k in 0..ts.len() {
            let lo = if k > 0 { rs[k - 1] } else { INF };
            let hi = if k + 1 < ts.len() { rs[k + 1] } else { INF };
            if rs[k] <= lo && rs[k] <= hi {
                let (mut a, mut b) = (ts[k.saturating_sub(1)], ts[(k + 1).min(ts.len() - 1)]);
                for _ in 0..100 {
                    let m1 = b - (b - a) * 0.6180339887498949;
                    let m2 = a + (b - a) * 0.6180339887498949;
                    if r(m1)? < r(m2)? {
                        b = m2;
                    } else {
                        a = m1;
                    }
                }
                let tm = (a + b) / 2.0;
                if r(tm)? < 1e-9 * size
                    && found.iter().all(|f| (tm - f.val).abs() > 1e-9 * (t1 - t0))
                {
                    found.push(Sing {
                        p: c.point(tm)?,
                        axis: 1,
                        val: tm,
                    });
                }
            }
        }
        Ok(found)
    }

    /// Whether the profile is its own mirror image across the axis, so that turning either
    /// side of it makes the same surface (an ellipse about its axis, not a circle beside it).
    fn is_mirrored(&self) -> R<bool> {
        let c = &*self.c;
        let (t0, t1) = c.range();
        if !(t0.is_finite() && t1.is_finite()) {
            return Ok(true);
        }
        for k in 0..16 {
            let p = c.point(t0 + (t1 - t0) * (k as f64 + 0.5) / 16.0)?;
            let q = add(self.a0, self.rot(sub(p, self.a0), PI)?);
            if norm(sub(c.point(c.invert(q, None)?)?, q)) > 1e-9 * (1.0 + norm(sub(p, self.a0))) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn ev(&self, u: f64, v: f64) -> R<(V3, V3, V3)> {
        if self.revolve {
            let (p, d) = self.c.eval(v)?;
            let w = self.rot(sub(p, self.a0), u)?;
            return Ok((add(self.a0, w), cross(self.a, w), self.rot(d, u)?));
        }
        let (p, d) = self.c.eval(u)?;
        Ok((add(p, mul(self.d, v)), d, self.d))
    }

    fn guess(&self, q: V3) -> R<Uv> {
        let c = &*self.c;
        if self.revolve {
            // the point turned about the axis into the profile's half-plane, and found on the
            // profile there
            // the half-plane of the profile's start (or of a point of it off the axis): the same
            // for every point, so that a face's loops are found on one sheet of the surface
            let (t0, t1) = c.range();
            let tries: Vec<f64> = if t0.is_finite() {
                vec![t0, (t0 + t1) / 2.0]
            } else {
                vec![1.0, -1.0, 10.0]
            };
            let mut radial = [0.0; 3];
            for t in tries {
                let w = sub(c.point(t)?, self.a0);
                radial = sub(w, mul(self.a, dot(w, self.a)));
                if norm(radial) > 1e-9 * (1.0 + norm(w)) {
                    break;
                }
            }
            let e = sub(q, self.a0);
            let h = dot(e, self.a);
            let rho = norm(sub(e, mul(self.a, h)));
            if norm(radial) > 0.0 {
                let p = add(add(self.a0, mul(self.a, h)), mul(unit(radial)?, rho));
                let t = c.invert(p, None)?;
                let w = sub(c.point(t)?, self.a0);
                let a1 = dot(w, self.y).atan2(dot(w, self.x));
                let a2 = dot(e, self.y).atan2(dot(e, self.x));
                return Ok((a2 - a1, t));
            }
        } else if !c.range().0.is_finite() {
            // a line's or a conic's parameter: the point moved back along the direction
            let dd = dot(self.d, self.d);
            let e = sub(q, mul(self.d, div(dot(sub(q, c.point(0.0)?), self.d), dd)?));
            let t = c.invert(e, None)?;
            return Ok((t, div(dot(sub(q, c.point(t)?), self.d), dd)?));
        }
        let (t0, t1) = c.range();
        let samples = match self.samples.get() {
            Some(s) => s,
            None => {
                let mut ts = vec![t0, t1];
                ts.extend(c.breaks().iter().copied());
                let ts = sorted_set(ts.into_iter().filter(|t| t0 <= *t && *t <= t1));
                let mut out = Vec::new();
                for w in ts.windows(2) {
                    for k in 0..8 {
                        out.push(w[0] + (w[1] - w[0]) * k as f64 / 8.0);
                    }
                }
                out.push(t1);
                self.samples.get_or_init(|| out)
            }
        };
        let mut best: Option<(f64, f64)> = None;
        for &t in samples {
            let p = c.point(t)?;
            let d = if self.revolve {
                let (w, e) = (sub(p, self.a0), sub(q, self.a0));
                let (h1, h2) = (dot(w, self.a), dot(e, self.a));
                let r1 = norm(sub(w, mul(self.a, h1)));
                let r2 = norm(sub(e, mul(self.a, h2)));
                (r1 - r2) * (r1 - r2) + (h1 - h2) * (h1 - h2)
            } else {
                let e = sub(q, p);
                let e = sub(e, mul(self.d, div(dot(e, self.d), dot(self.d, self.d))?));
                dot(e, e)
            };
            if best.is_none() || d < best.unwrap().0 {
                best = Some((d, t));
            }
        }
        let t = best.unwrap().1;
        if self.revolve {
            let (w, e) = (sub(c.point(t)?, self.a0), sub(q, self.a0));
            let a1 = dot(w, self.y).atan2(dot(w, self.x));
            let a2 = dot(e, self.y).atan2(dot(e, self.x));
            return Ok((a2 - a1, t));
        }
        Ok((
            t,
            div(dot(sub(q, c.point(t)?), self.d), dot(self.d, self.d))?,
        ))
    }
}

fn revolved(
    kind: Kind,
    periods: [Option<f64>; 2],
    angles: [bool; 2],
    nodes: [usize; 2],
) -> Surface {
    Surface {
        periods,
        domain: [(-INF, INF), (-INF, INF)],
        singular: RwLock::new(Vec::new()),
        breaks: [Vec::new(), Vec::new()],
        angles,
        nodes,
        kind,
    }
}

impl Surface {
    pub fn plane(f: Frame) -> Surface {
        revolved(
            Kind::Plane(f),
            [None, None],
            [false, false],
            [exact(1), exact(1)],
        )
    }

    pub fn cylinder(f: Frame, r: f64) -> Surface {
        revolved(
            Kind::Cylinder(f, r),
            [Some(TAU), None],
            [true, false],
            [16, exact(1)],
        )
    }

    pub fn cone(f: Frame, r: f64, angle: f64) -> R<Surface> {
        let k = angle.tan();
        if k == 0.0 {
            return unsupported("a cone of semi-angle zero");
        }
        let apex = -r / k;
        let sing = vec![Sing {
            p: add(f.o, mul(f.z, apex)),
            axis: 1,
            val: apex,
        }];
        let mut s = revolved(
            Kind::Cone(f, r, k),
            [Some(TAU), None],
            [true, false],
            [16, exact(1)],
        );
        s.singular = RwLock::new(sing);
        Ok(s)
    }

    pub fn sphere(f: Frame, r: f64) -> Surface {
        let sing = vec![
            Sing {
                p: add(f.o, mul(f.z, -r)),
                axis: 1,
                val: -PI / 2.0,
            },
            Sing {
                p: add(f.o, mul(f.z, r)),
                axis: 1,
                val: PI / 2.0,
            },
        ];
        let mut s = revolved(
            Kind::Sphere(f, r),
            [Some(TAU), None],
            [true, true],
            [16, 16],
        );
        s.domain = [(-INF, INF), (-PI / 2.0, PI / 2.0)];
        s.singular = RwLock::new(sing);
        s
    }

    pub fn torus(f: Frame, major: f64, r: f64, outer: Option<bool>) -> R<Surface> {
        let mut sing = Vec::new();
        if r >= major {
            // the surface meets its axis
            let x = div(-major, r)?;
            if !(-1.0..=1.0).contains(&x) {
                return Err(Err::Py(
                    "ValueError",
                    format!(
                        "expected a number in range from -1 up to 1, got {}",
                        float_repr(x)
                    ),
                ));
            }
            let a = x.acos();
            let h = r * a.sin();
            sing.push(Sing {
                p: add(f.o, mul(f.z, h)),
                axis: 1,
                val: a,
            });
            sing.push(Sing {
                p: add(f.o, mul(f.z, -h)),
                axis: 1,
                val: -a,
            });
        }
        let mut s = revolved(
            Kind::Torus(f, major, r, outer),
            [Some(TAU), Some(TAU)],
            [true, true],
            [16, 16],
        );
        s.singular = RwLock::new(sing);
        Ok(s)
    }

    pub fn bspline(
        pu: i64,
        pv: i64,
        grid: Vec<Vec<V3>>,
        weights: Option<Vec<Vec<f64>>>,
        u: Vec<f64>,
        v: Vec<f64>,
    ) -> R<Surface> {
        let (b, nodes, sing) = BSplineSurface::new(pu, pv, grid, weights, u, v)?;
        let breaks = |k: &Vec<f64>| {
            if k.len() > 2 {
                k[1..k.len() - 1].to_vec()
            } else {
                Vec::new()
            }
        };
        Ok(Surface {
            periods: b.periods,
            domain: b.domain,
            singular: RwLock::new(sing),
            breaks: [breaks(&b.knots[0]), breaks(&b.knots[1])],
            angles: [false, false],
            nodes,
            kind: Kind::BSpline(b),
        })
    }

    fn swept(curve: Arc<Curve>, revolve: Option<(V3, V3)>, extrude: V3) -> R<Surface> {
        let (t0, t1) = curve.range();
        let along = match curve.degree() {
            Some(1) => exact(1),
            Some(0) | None => 16,
            Some(d) => curved(d),
        };
        let mut sw = Swept {
            c: curve.clone(),
            revolve: revolve.is_some(),
            a0: [0.0; 3],
            a: [0.0; 3],
            x: [0.0; 3],
            y: [0.0; 3],
            mirrored: true,
            d: extrude,
            samples: OnceLock::new(),
        };
        if let Some((a0, a)) = revolve {
            sw.a0 = a0;
            sw.a = a;
            let mut along = along;
            if let Curve::Line { p, d, .. } = &*curve {
                if dot(cross(*d, a), sub(*p, a0)).abs()
                    > 1e-12 * norm(*d) * (1.0 + norm(sub(*p, a0)))
                {
                    along = 16; // a line askew of the axis turns a hyperboloid: its radius not linear
                }
            }
            let sing = sw.on_axis()?;
            // a frame about the axis, for inversion
            let x = if a[0].abs() < 0.9 {
                cross(a, [1.0, 0.0, 0.0])
            } else {
                cross(a, [0.0, 1.0, 0.0])
            };
            sw.x = unit(x)?;
            sw.y = cross(a, sw.x);
            sw.mirrored = sw.is_mirrored()?;
            Ok(Surface {
                periods: [Some(TAU), curve.period()],
                domain: [(-INF, INF), (t0, t1)],
                singular: RwLock::new(sing),
                breaks: [Vec::new(), curve.breaks().to_vec()],
                // a conic's parameter is an angle: its panels no more than a quarter turn
                angles: [true, curve.angular()],
                nodes: [16, along],
                kind: Kind::Swept(sw),
            })
        } else {
            Ok(Surface {
                periods: [curve.period(), None],
                domain: [(t0, t1), (-INF, INF)],
                singular: RwLock::new(Vec::new()),
                breaks: [curve.breaks().to_vec(), Vec::new()],
                angles: [curve.angular(), false],
                nodes: [along, exact(1)],
                kind: Kind::Swept(sw),
            })
        }
    }

    pub fn revolution(curve: Arc<Curve>, a0: V3, a: V3) -> R<Surface> {
        Surface::swept(curve, Some((a0, a)), [0.0; 3])
    }

    pub fn extrusion(curve: Arc<Curve>, d: V3) -> R<Surface> {
        Surface::swept(curve, None, d)
    }

    /// A surface moved along its unit normal by d. Its derivatives take the normal's by central
    /// differences, to about 1e-10.
    pub fn offset(basis: Arc<Surface>, d: f64) -> Surface {
        Surface {
            periods: basis.periods,
            domain: basis.domain,
            singular: RwLock::new(Vec::new()),
            breaks: basis.breaks.clone(),
            angles: basis.angles,
            nodes: [16, 16],
            kind: Kind::Offset(basis, d),
        }
    }

    pub fn singular(&self) -> Vec<Sing> {
        self.singular.read().unwrap().clone()
    }

    /// Whether a pole the surface did not know of may be recorded on it (`collapsed`).
    pub fn learns_poles(&self) -> bool {
        matches!(
            self.kind,
            Kind::BSpline(_) | Kind::Swept(_) | Kind::Offset(..)
        )
    }

    /// How many knots it has along each parameter (none, but a B-spline).
    pub fn spans(&self) -> [usize; 2] {
        match &self.kind {
            Kind::BSpline(b) => [b.knots[0].len(), b.knots[1].len()],
            _ => [0, 0],
        }
    }

    /// Whether it is a surface of revolution whose profile is not its own mirror image across
    /// the axis.
    pub fn not_mirrored(&self) -> bool {
        match &self.kind {
            Kind::Swept(s) => s.revolve && !s.mirrored,
            _ => false,
        }
    }

    fn frame(&self) -> &Frame {
        match &self.kind {
            Kind::Plane(f)
            | Kind::Cylinder(f, _)
            | Kind::Cone(f, ..)
            | Kind::Sphere(f, _)
            | Kind::Torus(f, ..) => f,
            _ => unreachable!(),
        }
    }

    /// A surface of revolution's profile at v: (ρ, h, ρ', h').
    #[inline(always)]
    fn profile(&self, v: f64) -> R<(f64, f64, f64, f64)> {
        Ok(match &self.kind {
            Kind::Cylinder(_, r) => (*r, v, 0.0, 1.0),
            Kind::Cone(_, r, k) => (r + v * k, v, *k, 1.0),
            Kind::Sphere(_, r) => {
                let (c, s) = (cosine(v)?, sine(v)?);
                (r * c, r * s, -r * s, r * c)
            }
            Kind::Torus(_, major, r, _) => {
                let (c, s) = (cosine(v)?, sine(v)?);
                (major + r * c, r * s, -r * s, r * c)
            }
            _ => unreachable!(),
        })
    }

    fn polar(&self, q: V3) -> (f64, f64, f64) {
        let f = self.frame();
        let d = sub(q, f.o);
        let (x, y, z) = (dot(d, f.x), dot(d, f.y), dot(d, f.z));
        (y.atan2(x), hypot(x, y), z)
    }

    pub fn ev(&self, u: f64, v: f64) -> R<(V3, V3, V3)> {
        match &self.kind {
            Kind::Plane(f) => Ok((lin3(1.0, f.o, u, f.x, v, f.y), f.x, f.y)),
            Kind::BSpline(b) => {
                let (u, v) = b.wrap(u, v)?;
                b.ev_raw(u, v)
            }
            Kind::Swept(s) => s.ev(u, v),
            Kind::Offset(b, d) => {
                let (p, su, sv) = b.ev(u, v)?;
                let n = unit(cross(su, sv))?;
                let nrm = |u: f64, v: f64| -> R<V3> {
                    let (_, su, sv) = b.ev(u, v)?;
                    unit(cross(su, sv))
                };
                let h = 1e-5;
                let nu = mul(sub(nrm(u + h, v)?, nrm(u - h, v)?), 1.0 / (2.0 * h));
                let nv = mul(sub(nrm(u, v + h)?, nrm(u, v - h)?), 1.0 / (2.0 * h));
                Ok((
                    add(p, mul(n, *d)),
                    add(su, mul(nu, *d)),
                    add(sv, mul(nv, *d)),
                ))
            }
            _ => {
                let f = self.frame();
                let (r, h, dr, dh) = self.profile(v)?;
                let (cu, su) = (cosine(u)?, sine(u)?);
                let radial = lin2(cu, f.x, su, f.y);
                Ok((
                    lin3(1.0, f.o, r, radial, h, f.z),
                    lin2(-r * su, f.x, r * cu, f.y),
                    lin2(dr, radial, dh, f.z),
                ))
            }
        }
    }

    /// The points and normals (S_u × S_v) at (u, v) for each v: (x, y, z, nx, ny, nz).
    pub fn col(&self, u: f64, vs: &[f64], out: &mut Vec<[f64; 6]>) -> R<()> {
        match &self.kind {
            Kind::Plane(f) => {
                out.clear();
                let (o, x, y, z) = (f.o, f.x, f.y, f.z);
                for &v in vs {
                    out.push([
                        o[0] + u * x[0] + v * y[0],
                        o[1] + u * x[1] + v * y[1],
                        o[2] + u * x[2] + v * y[2],
                        z[0],
                        z[1],
                        z[2],
                    ]);
                }
                Ok(())
            }
            Kind::BSpline(b) => b.line(0, u, vs, out),
            Kind::Swept(_) | Kind::Offset(..) => {
                out.clear();
                for &v in vs {
                    let (p, su, sv) = self.ev(u, v)?;
                    let n = normal(su, sv);
                    out.push([p[0], p[1], p[2], n[0], n[1], n[2]]);
                }
                Ok(())
            }
            // S = O + ρ(v) e + h(v) Z, e = cos u X + sin u Y; S_u × S_v = ρ (h' e - ρ' Z)
            _ => {
                out.clear();
                let f = self.frame();
                let (c, s) = (cosine(u)?, sine(u)?);
                let (o, x, y, z) = (f.o, f.x, f.y, f.z);
                let (ex, ey, ez) = (
                    c * x[0] + s * y[0],
                    c * x[1] + s * y[1],
                    c * x[2] + s * y[2],
                );
                for &v in vs {
                    let (r, h, dr, dh) = self.profile(v)?;
                    out.push([
                        o[0] + r * ex + h * z[0],
                        o[1] + r * ey + h * z[1],
                        o[2] + r * ez + h * z[2],
                        r * (dh * ex - dr * z[0]),
                        r * (dh * ey - dr * z[1]),
                        r * (dh * ez - dr * z[2]),
                    ]);
                }
                Ok(())
            }
        }
    }

    /// As `col`, at (u, v) for each u.
    pub fn row(&self, v: f64, us: &[f64], out: &mut Vec<[f64; 6]>) -> R<()> {
        match &self.kind {
            Kind::Plane(f) => {
                out.clear();
                let (o, x, y, z) = (f.o, f.x, f.y, f.z);
                for &u in us {
                    out.push([
                        o[0] + u * x[0] + v * y[0],
                        o[1] + u * x[1] + v * y[1],
                        o[2] + u * x[2] + v * y[2],
                        z[0],
                        z[1],
                        z[2],
                    ]);
                }
                Ok(())
            }
            Kind::BSpline(b) => b.line(1, v, us, out),
            Kind::Swept(_) | Kind::Offset(..) => {
                out.clear();
                for &u in us {
                    let (p, su, sv) = self.ev(u, v)?;
                    let n = normal(su, sv);
                    out.push([p[0], p[1], p[2], n[0], n[1], n[2]]);
                }
                Ok(())
            }
            _ => {
                out.clear();
                let f = self.frame();
                let (r, h, dr, dh) = self.profile(v)?;
                let (o, x, y, z) = (f.o, f.x, f.y, f.z);
                for &u in us {
                    let (c, s) = (cosine(u)?, sine(u)?);
                    let (ex, ey, ez) = (
                        c * x[0] + s * y[0],
                        c * x[1] + s * y[1],
                        c * x[2] + s * y[2],
                    );
                    out.push([
                        o[0] + r * ex + h * z[0],
                        o[1] + r * ey + h * z[1],
                        o[2] + r * ez + h * z[2],
                        r * (dh * ex - dr * z[0]),
                        r * (dh * ey - dr * z[1]),
                        r * (dh * ez - dr * z[2]),
                    ]);
                }
                Ok(())
            }
        }
    }

    /// The (u, v) of the point `q` of the surface (`guess`: a (u, v) near it, or none).
    pub fn invert(&self, q: V3, guess: Option<Uv>) -> R<Uv> {
        match &self.kind {
            Kind::Plane(f) => {
                let d = sub(q, f.o);
                Ok((dot(d, f.x), dot(d, f.y)))
            }
            Kind::Cylinder(..) => {
                let (u, _, z) = self.polar(q);
                Ok((u, z))
            }
            Kind::Cone(_, r, k) => {
                let (mut u, _, z) = self.polar(q);
                // the side of the apex the point is on: its radius there is negative, its angle
                // turned
                if r + z * k < 0.0 {
                    u += PI;
                }
                Ok((u, z))
            }
            Kind::Sphere(..) => {
                let (u, rho, z) = self.polar(q);
                Ok((u, z.atan2(rho)))
            }
            Kind::Torus(_, major, r, outer) => {
                let (major, r) = (*major, *r);
                let (u, rho, z) = self.polar(q);
                let apple = (u, z.atan2(rho - major));
                if r < major || *outer == Some(true) {
                    return Ok(apple);
                }
                // within the axis: the radius R + r cos v is negative there, the angle turned
                let lemon = (u + PI, z.atan2(-rho - major));
                if *outer == Some(false) {
                    return Ok(lemon);
                }
                // the sheet the point is on: (ρ - R)² + z² = r² on the apple, (ρ + R)² + z² = r²
                // within
                let off_apple = (hypot(rho - major, z) - r).abs();
                let off_lemon = (hypot(rho + major, z) - r).abs();
                let Some(guess) = guess else {
                    return Ok(if off_apple <= off_lemon { apple } else { lemon });
                };
                if (off_apple - off_lemon).abs() > 1e-9 * r {
                    return Ok(if off_apple <= off_lemon { apple } else { lemon });
                }
                let off = |a: Uv| -> R<f64> {
                    let du = pymod(a.0 - guess.0 + PI, TAU)? - PI;
                    let dv = pymod(a.1 - guess.1 + PI, TAU)? - PI;
                    Ok(du * du + dv * dv)
                };
                Ok(if off(lemon)? < off(apple)? {
                    lemon
                } else {
                    apple
                })
            }
            Kind::BSpline(b) => b.invert(self, q, guess),
            Kind::Swept(s) => {
                let (u, v) = match guess {
                    Some(g) => g,
                    None => s.guess(q)?,
                };
                newton2(self, q, u, v)
            }
            Kind::Offset(b, _) => {
                let (u, v) = match guess {
                    Some(g) => g,
                    None => b.invert(q, None)?,
                };
                newton2(self, q, u, v)
            }
        }
    }
}

/// A surface as a face's integrals see it: itself, or with its parameters swapped, so that the
/// inner integral runs along u.
#[derive(Clone)]
pub struct View {
    pub s: Arc<Surface>,
    pub transposed: bool,
}

impl View {
    pub fn periods(&self) -> [Option<f64>; 2] {
        let p = self.s.periods;
        if self.transposed { [p[1], p[0]] } else { p }
    }

    pub fn domain(&self) -> [(f64, f64); 2] {
        let d = self.s.domain;
        if self.transposed { [d[1], d[0]] } else { d }
    }

    pub fn breaks(&self, axis: usize) -> &[f64] {
        &self.s.breaks[if self.transposed { 1 - axis } else { axis }]
    }

    pub fn angles(&self) -> [bool; 2] {
        let a = self.s.angles;
        if self.transposed { [a[1], a[0]] } else { a }
    }

    pub fn nodes(&self) -> [usize; 2] {
        let n = self.s.nodes;
        if self.transposed { [n[1], n[0]] } else { n }
    }

    pub fn singular(&self) -> Vec<Sing> {
        let mut s = self.s.singular();
        if self.transposed {
            for x in &mut s {
                x.axis = 1 - x.axis;
            }
        }
        s
    }

    pub fn ev(&self, u: f64, v: f64) -> R<(V3, V3, V3)> {
        if self.transposed {
            let (p, su, sv) = self.s.ev(v, u)?;
            Ok((p, sv, su))
        } else {
            self.s.ev(u, v)
        }
    }

    pub fn invert(&self, q: V3, guess: Option<Uv>) -> R<Uv> {
        if self.transposed {
            let (u, v) = self.s.invert(q, guess.map(|g| (g.1, g.0)))?;
            Ok((v, u))
        } else {
            self.s.invert(q, guess)
        }
    }

    /// The points and normals at (u, v) for each v; a transposed surface's normal,
    /// S_a × S_b = S_v × S_u, the other way.
    pub fn col(&self, u: f64, vs: &[f64], out: &mut Vec<[f64; 6]>) -> R<()> {
        if self.transposed {
            self.s.row(u, vs, out)?;
            for p in out.iter_mut() {
                p[3] = -p[3];
                p[4] = -p[4];
                p[5] = -p[5];
            }
            Ok(())
        } else {
            self.s.col(u, vs, out)
        }
    }
}
