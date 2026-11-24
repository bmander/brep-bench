//! A face's loops on its surface's (u, v) plane.
use crate::curve::*;
use crate::geom::*;
use crate::model::*;
use crate::py::*;
use crate::step::*;
use crate::surface::*;
use std::sync::Arc;

/// `x` moved by whole periods to be nearest `r`.
#[inline]
pub fn nearest(x: f64, r: f64, period: Option<f64>) -> R<f64> {
    match period {
        None => Ok(x),
        Some(p) => Ok(x + round((r - x) / p)? * p),
    }
}

/// A piece of a loop on the (u, v) plane: along an edge's curve from t0 to t1, its points
/// inverted onto the surface and followed continuously from a reference path, or straight from
/// one point to another (`line`), along a singular line.
pub struct Path {
    pub c: Option<Arc<Curve>>,
    pub t0: f64,
    pub t1: f64,
    pub line: Option<(Uv, Uv)>,
    /// (t, (u, v)): the path, sampled, followed continuously.
    pub rf: Vec<(f64, Uv)>,
}

impl Path {
    pub fn straight(a: Uv, b: Uv) -> Path {
        Path {
            c: None,
            t0: 0.0,
            t1: 1.0,
            line: Some((a, b)),
            rf: Vec::new(),
        }
    }

    /// (u, v, du/dt, dv/dt) at `t`, on the surface `s`.
    pub fn uv(&self, s: &View, t: f64) -> R<(f64, f64, f64, f64)> {
        let (u, v, du, dv, _) = self.uv_square(s, t)?;
        Ok((u, v, du, dv))
    }

    /// (u, v, du/dt, dv/dt) at `t`, on the surface `s`, and how square the surface's rates
    /// stand there: the square of the sine of the angle between S_u and S_v (1 along a line of
    /// the (u, v) plane, which asks nothing of them).
    pub fn uv_square(&self, s: &View, t: f64) -> R<(f64, f64, f64, f64, f64)> {
        if let Some((a, b)) = self.line {
            let (du, dv) = (b.0 - a.0, b.1 - a.1);
            return Ok((a.0 + t * du, a.1 + t * dv, du, dv, 1.0));
        }
        let c = self.c.as_ref().unwrap();
        let (q, d) = c.eval(t)?;
        let g = self.guess(t);
        let (u, v) = s.invert(q, Some(g))?;
        let periods = s.periods();
        let u = nearest(u, g.0, periods[0])?;
        let v = nearest(v, g.1, periods[1])?;
        let (_, su, sv) = s.ev(u, v)?;
        let (ee, ff, gg) = (dot(su, su), dot(su, sv), dot(sv, sv));
        let (a, b) = (dot(su, d), dot(sv, d));
        let det = ee * gg - ff * ff;
        if det <= 0.0 {
            return Ok((u, v, 0.0, 0.0, if ee * gg > 0.0 { 0.0 } else { 1.0 }));
        }
        Ok((
            u,
            v,
            (gg * a - ff * b) / det,
            (ee * b - ff * a) / det,
            det / (ee * gg),
        ))
    }

    fn guess(&self, t: f64) -> Uv {
        let rf = &self.rf;
        let (mut lo, mut hi) = (0, rf.len() - 1);
        let whole = rf[hi].0 - rf[0].0;
        if (t - rf[0].0) * whole <= 0.0 {
            return rf[0].1;
        }
        while hi - lo > 1 {
            let mid = (lo + hi) / 2;
            if (t - rf[mid].0) * whole >= 0.0 {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        let ((ta, a), (tb, b)) = (rf[lo], rf[hi]);
        let f = if tb != ta { (t - ta) / (tb - ta) } else { 0.0 };
        (a.0 + f * (b.0 - a.0), a.1 + f * (b.1 - a.1))
    }

    /// The path's points on the (u, v) plane, in order: a line's two ends, else its samples.
    pub fn points(&self) -> impl Iterator<Item = Uv> + '_ {
        let ends = self.line.map(|(a, b)| [a, b]);
        ends.into_iter()
            .flatten()
            .chain(self.rf.iter().map(|x| x.1))
    }

    pub fn start(&self) -> Uv {
        match self.line {
            Some((a, _)) => a,
            None => self.rf[0].1,
        }
    }

    pub fn end(&self) -> Uv {
        match self.line {
            Some((_, b)) => b,
            None => self.rf[self.rf.len() - 1].1,
        }
    }

    fn shift(&mut self, du: f64, dv: f64) {
        match &mut self.line {
            Some((a, b)) => {
                *a = (a.0 + du, a.1 + dv);
                *b = (b.0 + du, b.1 + dv);
            }
            None => {
                for (_, p) in &mut self.rf {
                    *p = (p.0 + du, p.1 + dv);
                }
            }
        }
    }

    /// The path with its surface's parameters swapped.
    pub fn transpose(&mut self) {
        if let Some((a, b)) = &mut self.line {
            *a = (a.1, a.0);
            *b = (b.1, b.0);
        } else {
            for (_, p) in &mut self.rf {
                *p = (p.1, p.0);
            }
        }
    }
}

/// The edge's curve from t0 to t1 on the surface `s`: its sampled path in (u, v), followed
/// continuously, refined where it turns fast; split where it passes through a singular point of
/// the surface. The paths, each with its reference path.
fn follow(
    s: &Arc<Surface>,
    curve: &Arc<Curve>,
    guide: Option<&Arc<Curve>>,
    t0: f64,
    t1: f64,
    scale: f64,
    mut seed: Option<Uv>,
) -> R<Vec<Path>> {
    // where the curve passes through a singular point, it is split
    let mut cuts = Vec::new();
    for sing in s.singular() {
        let p = sing.p;
        let ts: Vec<f64> = (0..65).map(|k| t0 + (t1 - t0) * k as f64 / 64.0).collect();
        let mut ds = Vec::with_capacity(65);
        for t in &ts {
            ds.push(norm(sub(curve.point(*t)?, p)));
        }
        for k in 1..64 {
            if ds[k] <= ds[k - 1] && ds[k] <= ds[k + 1] {
                let (mut a, mut b) = (ts[k - 1], ts[k + 1]);
                for _ in 0..80 {
                    // golden section
                    let m1 = b - (b - a) * 0.6180339887498949;
                    let m2 = a + (b - a) * 0.6180339887498949;
                    if norm(sub(curve.point(m1)?, p)) < norm(sub(curve.point(m2)?, p)) {
                        b = m2;
                    } else {
                        a = m1;
                    }
                }
                let t = (a + b) / 2.0;
                if norm(sub(curve.point(t)?, p)) < 1e-7 * scale {
                    cuts.push(t);
                }
            }
        }
    }
    let whole = (t1 - t0).abs();
    let cuts = sorted(
        cuts.into_iter()
            .filter(|t| pymin((t - t0).abs(), (t - t1).abs()) > 1e-9 * whole)
            .collect(),
    );
    // one singular point may be several lines (a horn torus's centre): one cut there
    let mut bounds = vec![t0];
    for (k, t) in cuts.iter().enumerate() {
        if k == 0 || t - cuts[k - 1] > 1e-9 * whole {
            bounds.push(*t);
        }
    }
    bounds.push(t1);
    let mut pieces: Vec<Path> = Vec::new();
    for w in bounds.windows(2) {
        let p = piece(s, curve, guide, w[0], w[1], scale, seed)?;
        seed = Some(p.end());
        pieces.push(p);
    }
    Ok(pieces)
}

fn at_singular(sing: &[Sing], q: V3, scale: f64) -> Option<(usize, f64)> {
    sing.iter()
        .find(|x| norm(sub(q, x.p)) < 1e-7 * scale)
        .map(|x| (x.axis, x.val))
}

/// One piece of an edge, through no singular point but perhaps at its ends. Where the file gives
/// the edge's curve on the surface (`guide`, a spline surface's), each sample's foot is sought from
/// its point on that curve rather than from the last foot: on a sheet folded back past its face,
/// the feet followed from one another crossed onto the layer beyond the fold, and the loop's path
/// broke there.
fn piece(
    s: &Arc<Surface>,
    curve: &Arc<Curve>,
    guide: Option<&Arc<Curve>>,
    t0: f64,
    t1: f64,
    scale: f64,
    seed: Option<Uv>,
) -> R<Path> {
    let sing = s.singular();
    let periods = s.periods;
    // a sample a piece of the curve (fast turns are refined below), at least 16
    let (tlo, thi) = (pymin(t0, t1), pymax(t0, t1));
    let n = (16
        + curve
            .breaks()
            .iter()
            .filter(|b| tlo < **b && **b < thi)
            .count())
    .min(4000);
    let ts: Vec<f64> = (0..=n)
        .map(|k| t0 + (t1 - t0) * k as f64 / n as f64)
        .collect();
    let ends = [
        at_singular(&sing, curve.point(t0)?, scale),
        at_singular(&sing, curve.point(t1)?, scale),
    ];
    // within a hair of a singular end the angles are rounding's: no samples there (the end itself
    // is the limit from inside, below)
    let clear = |t: f64| -> R<bool> {
        if sing.is_empty() {
            return Ok(true);
        }
        let q = curve.point(t)?;
        Ok(sing.iter().all(|x| norm(sub(q, x.p)) > 1e-6 * scale))
    };
    let mut kept = Vec::with_capacity(ts.len());
    for (k, t) in ts.iter().enumerate() {
        if k == 0 || k == ts.len() - 1 || clear(*t)? {
            kept.push(*t);
        }
    }
    let ts = kept;
    // followed continuously: from where the loop's last edge ended, where that is given and this
    // edge does not start at a pole (on a surface that comes near itself, the nearest of its
    // samples may be on the wrong fold); else from the middle of the edge outwards both ways, away
    // from any pole at its ends (near one, inversion from a sample can settle on the pole's line)
    let walk = |order: &mut dyn Iterator<Item = f64>, mut prev: Option<Uv>| -> R<Vec<(f64, Uv)>> {
        let mut out = Vec::new();
        for t in order {
            let from = match guide {
                Some(g) => {
                    let q = g.point(t)?;
                    Some((q[0], q[1]))
                }
                None => prev,
            };
            let mut uv = s.invert(curve.point(t)?, from)?;
            if let Some(prev) = from {
                uv = (
                    nearest(uv.0, prev.0, periods[0])?,
                    nearest(uv.1, prev.1, periods[1])?,
                );
            }
            out.push((t, uv));
            prev = Some(uv);
        }
        Ok(out)
    };
    let mut pts = if seed.is_some() && ends[0].is_none() {
        walk(&mut ts.iter().copied(), seed)?
    } else {
        let mid = ts.len() / 2;
        let ahead = walk(&mut ts[mid..].iter().copied(), None)?;
        let mut back = if mid > 0 {
            if ahead.is_empty() {
                return index_error();
            }
            walk(&mut ts[..mid].iter().rev().copied(), Some(ahead[0].1))?
        } else {
            Vec::new()
        };
        back.reverse();
        back.extend(ahead);
        back
    };
    // refine where it turns fast (near a pole the angle sweeps): bisect
    let angles = s.angles;
    let mut k = 0;
    while k + 1 < pts.len() && pts.len() < 5000 {
        let ((ta, a), (tb, b)) = (pts[k], pts[k + 1]);
        let jump = pymax(
            if angles[0] { (a.0 - b.0).abs() } else { 0.0 },
            if angles[1] { (a.1 - b.1).abs() } else { 0.0 },
        );
        let tm = (ta + tb) / 2.0;
        if jump > 0.3 && (tb - ta).abs() > 1e-12 * (t1 - t0).abs() && clear(tm)? {
            let uv = s.invert(curve.point(tm)?, Some(a))?;
            let uv = (
                nearest(uv.0, a.0, periods[0])?,
                nearest(uv.1, a.1, periods[1])?,
            );
            pts.insert(k + 1, (tm, uv));
        } else {
            k += 1;
        }
    }
    // an end at a singular point: the limit along the curve, the singular parameter exact
    for (first, end) in [(true, ends[0]), (false, ends[1])] {
        let Some((axis, val)) = end else { continue };
        let last = pts.len() - 1;
        let i = if first { 0 } else { last };
        let t = pts[i].0;
        let h = (t1 - t0) * 1e-5 * if first { 1.0 } else { -1.0 };
        // (Python's pts[1] and pts[-2], which a path of one point reads from its other end)
        let g = if first {
            match pts.get(1) {
                Some(p) => p.1,
                None => return index_error(),
            }
        } else {
            if pts.len() < 2 {
                return index_error();
            }
            pts[last - 1].1
        };
        let a = s.invert(curve.point(t + h)?, Some(g))?;
        let a = (
            nearest(a.0, g.0, periods[0])?,
            nearest(a.1, g.1, periods[1])?,
        );
        let b = s.invert(curve.point(t + 2.0 * h)?, Some(a))?;
        let b = (
            nearest(b.0, a.0, periods[0])?,
            nearest(b.1, a.1, periods[1])?,
        );
        let mut lim = [2.0 * a.0 - b.0, 2.0 * a.1 - b.1];
        lim[axis] = nearest(val, lim[axis], periods[axis])?;
        pts[i] = (t, (lim[0], lim[1]));
    }
    Ok(Path {
        c: Some(curve.clone()),
        t0,
        t1,
        line: None,
        rf: pts,
    })
}

fn closed_edge(m: &Model, sv: &Val, ev: &Val) -> R<bool> {
    Ok(sv == ev || norm(sub(m.point(sv)?, m.point(ev)?)) < 1e-12)
}

/// The parameters of the edge from its start vertex to its end vertex on its curve, in the
/// direction `same_sense` says.
fn edge_range(m: &Model, curve: &Curve, sv: &Val, ev: &Val, same_sense: bool) -> R<(f64, f64)> {
    let (ps, pe) = (m.point(sv)?, m.point(ev)?);
    let (mut ts, mut te) = (curve.invert(ps, None)?, curve.invert(pe, None)?);
    if let Some(p) = curve.period() {
        if closed_edge(m, sv, ev)? {
            te = if same_sense { ts + p } else { ts - p };
        } else if same_sense {
            te = ts + pymod(te - ts, p)?;
        } else {
            te = ts - pymod(ts - te, p)?;
        }
    } else if closed_edge(m, sv, ev)? {
        // a closed curve that is not periodic: end to end
        let (t0, t1) = curve.range();
        (ts, te) = if same_sense { (t0, t1) } else { (t1, t0) };
    }
    Ok((ts, te))
}

pub fn newell(points: &[V3]) -> V3 {
    let mut n = [0.0; 3];
    for (k, a) in points.iter().enumerate() {
        let b = points[(k + 1) % points.len()];
        n[0] += (a[1] - b[1]) * (a[2] + b[2]);
        n[1] += (a[2] - b[2]) * (a[0] + b[0]);
        n[2] += (a[0] - b[0]) * (a[1] + b[1]);
    }
    n
}

/// A polygon's loop on the plane `s`: straight paths between its points.
fn poly_paths(s: &Arc<Surface>, pts: &[V3], scale: f64) -> R<Vec<Path>> {
    let mut paths = Vec::new();
    for (k, a) in pts.iter().enumerate() {
        let b = pts[(k + 1) % pts.len()];
        if norm(sub(*a, b)) == 0.0 {
            continue;
        }
        let c = Arc::new(Curve::line(*a, sub(b, *a)));
        paths.push(piece(s, &c, None, 0.0, 1.0, scale, None)?);
    }
    Ok(paths)
}

/// A loop's paths and how many times it winds round each period.
pub type Loop = (Vec<Path>, (f64, f64));

/// The loops of a face on its surface: (paths, (winding in u, winding in v)).
pub fn face_loops(m: &Model, f: &Val, s: &Arc<Surface>, scale: f64) -> R<Vec<Loop>> {
    let fa = m.one(f)?;
    let bounds = fa.at(1)?;
    let mut loops = Vec::new();
    for b in bounds.items()? {
        let ba = m.one(b)?;
        if !(ba.is("FACE_OUTER_BOUND") || ba.is("FACE_BOUND")) {
            return unsupported(format!("a face bound {}", ba.named()));
        }
        let (lp, orient) = (ba.at(1)?, !ba.at(2)?.is("F"));
        let la = m.one(lp)?;
        let mut paths;
        if la.is("VERTEX_LOOP") {
            continue;
        }
        if la.is("POLY_LOOP") {
            let mut pts = Vec::new();
            for p in la.at(1)?.items()? {
                pts.push(m.point(p)?);
            }
            if !orient {
                pts.reverse();
            }
            paths = poly_paths(s, &pts, scale)?;
        } else if la.is("EDGE_LOOP") {
            paths = Vec::new();
            let mut last: Option<Val> = None;
            let mut first_vertex: Option<Val> = None;
            for oe in la.at(1)?.items()? {
                let oa = m.one(oe)?;
                if !oa.is("ORIENTED_EDGE") {
                    return unsupported(format!("an edge {}", oa.named()));
                }
                let (e, eo) = (oa.at(3)?, !oa.at(4)?.is("F"));
                let ea = m.one(e)?;
                if !ea.is("EDGE_CURVE") {
                    return unsupported(format!("an edge {}", ea.named()));
                }
                let (sv, ev, geom, same) = (ea.at(1)?, ea.at(2)?, ea.at(3)?, !ea.at(4)?.is("F"));
                let ends = if eo { (sv, ev) } else { (ev, sv) }; // as the loop runs, before its bound turns it
                if let Some(last) = &last {
                    if norm(sub(m.point(last)?, m.point(ends.0)?)) > 1e-6 * scale {
                        return Err(Err::Invalid(
                            "a loop whose edges do not meet end to start".into(),
                        ));
                    }
                }
                last = Some(ends.1.clone());
                if first_vertex.is_none() {
                    first_vertex = Some(ends.0.clone());
                }
                let curve = m.curve(geom)?;
                if let Curve::Line { dd, .. } = &*curve {
                    if *dd == 0.0 || closed_edge(m, sv, ev)? {
                        continue; // an edge with no extent: a point
                    }
                }
                let (mut ts, mut te) = edge_range(m, &curve, sv, ev, same)?;
                if eo != orient {
                    (ts, te) = (te, ts);
                }
                let seed = paths.last().map(|p: &Path| p.end());
                // (on a spline surface, the edge's curve on it where the file gives it)
                let guide = match s.kind {
                    Kind::BSpline(_) => m.pcurve_on(geom, fa.at(2)?)?,
                    _ => None,
                };
                paths.extend(follow(s, &curve, guide.as_ref(), ts, te, scale, seed)?);
            }
            if let (Some(last), Some(first)) = (&last, &first_vertex) {
                if norm(sub(m.point(last)?, m.point(first)?)) > 1e-6 * scale {
                    return Err(Err::Invalid("a loop whose edges do not close".into()));
                }
            }
            if paths.is_empty() {
                continue; // a loop of points only
            }
            if !orient {
                paths.reverse();
            }
        } else {
            return unsupported(format!("a loop {}", la.named()));
        }
        loops.push(join(s, paths, scale)?);
    }
    Ok(loops)
}

/// Whether nine points along the line of the surface at `fixed` of the other parameter, from
/// `lo` to `hi` of `axis`, are one point: the point, if so.
fn one_point(s: &Surface, axis: usize, fixed: f64, lo: f64, hi: f64, scale: f64) -> R<Option<V3>> {
    let mut ps = Vec::with_capacity(9);
    for k in 0..9 {
        let x = lo + (hi - lo) * k as f64 / 8.0;
        ps.push(if axis == 0 {
            s.ev(x, fixed)?.0
        } else {
            s.ev(fixed, x)?.0
        });
    }
    let mut far = norm(sub(ps[0], ps[0]));
    for q in &ps[1..] {
        far = pymax(far, norm(sub(*q, ps[0])));
    }
    Ok(if far < 1e-7 * scale {
        Some(ps[0])
    } else {
        None
    })
}

/// Record a pole the surface did not know of at the point `uv` of a loop: a line of the surface
/// through it, inside its domain, collapsed to that point (a cone's apex where the surface runs
/// on past it).
fn collapsed(s: &Surface, uv: Uv, scale: f64) -> R<()> {
    if !s.learns_poles() {
        return Ok(());
    }
    for axis in 0..2 {
        let fixed = if axis == 0 { uv.1 } else { uv.0 };
        if s.singular()
            .iter()
            .any(|x| x.axis == 1 - axis && (x.val - fixed).abs() < 1e-9)
        {
            continue;
        }
        let (lo, hi) = s.domain[axis];
        if !(lo.is_finite() && hi.is_finite()) {
            continue;
        }
        if let Some(p) = one_point(s, axis, fixed, lo, hi, scale)? {
            s.singular.write().unwrap().push(Sing {
                p,
                axis: 1 - axis,
                val: fixed,
            });
        }
    }
    Ok(())
}

/// A loop's paths joined end to start on the (u, v) plane: each moved by whole periods to
/// meet the last, a path along a singular line inserted where the loop passes through a pole;
/// and how many times the loop winds round each period.
fn join(s: &Arc<Surface>, paths: Vec<Path>, scale: f64) -> R<Loop> {
    if paths.is_empty() {
        return unsupported("an empty loop");
    }
    for p in &paths {
        collapsed(s, p.start(), scale)?;
    }
    let periods = s.periods;
    let mut out: Vec<Path> = Vec::with_capacity(paths.len() + 1);
    for (k, mut p) in paths.into_iter().enumerate() {
        if k == 0 {
            out.push(p);
            continue;
        }
        let prev = out[out.len() - 1].end();
        let st = p.start();
        let du = nearest(st.0, prev.0, periods[0])? - st.0;
        let dv = nearest(st.1, prev.1, periods[1])? - st.1;
        p.shift(du, dv);
        gap(s, &mut out, prev, p.start(), scale)?;
        out.push(p);
    }
    // closing: the start, moved by whole periods to be nearest the end
    let (end, st) = (out[out.len() - 1].end(), out[0].start());
    let nu = match periods[0] {
        Some(p) => round((end.0 - st.0) / p)?,
        None => 0.0,
    };
    let nv = match periods[1] {
        Some(p) => round((end.1 - st.1) / p)?,
        None => 0.0,
    };
    let target = (
        st.0 + nu * periods[0].unwrap_or(0.0),
        st.1 + nv * periods[1].unwrap_or(0.0),
    );
    gap(s, &mut out, end, target, scale)?;
    Ok((out, (nu, nv)))
}

/// Join `a` to `b`: nothing if they meet, else a straight path along a singular line.
fn gap(s: &Arc<Surface>, out: &mut Vec<Path>, a: Uv, b: Uv, scale: f64) -> R<()> {
    let (du, dv) = (b.0 - a.0, b.1 - a.1);
    if du.abs() < 1e-7 && dv.abs() < 1e-7 {
        return Ok(());
    }
    let s1 = s.ev(a.0, a.1)?.0;
    for x in s.singular() {
        if norm(sub(s1, x.p)) < 1e-6 * scale {
            out.push(Path::straight(a, b));
            return Ok(());
        }
    }
    // a line of the surface collapsed to a point inside its domain (a cone's apex where the surface
    // runs on past it): a pole the surface did not know of
    for axis in 0..2 {
        let (d, other) = if axis == 0 { (du, dv) } else { (dv, du) };
        if d.abs() > 1e-7 && other.abs() < 1e-7 {
            let fixed = if axis == 0 { a.1 } else { a.0 };
            let (mut lo, mut hi) = s.domain[axis];
            if !(lo.is_finite() && hi.is_finite()) {
                let (x, y) = if axis == 0 { (a.0, b.0) } else { (a.1, b.1) };
                (lo, hi) = (pymin(x, y), pymax(x, y));
            }
            if let Some(p) = one_point(s, axis, fixed, lo, hi, scale)? {
                s.singular.write().unwrap().push(Sing {
                    p,
                    axis: 1 - axis,
                    val: fixed,
                });
                out.push(Path::straight(a, b));
                return Ok(());
            }
        }
    }
    if norm(sub(s1, s.ev(b.0, b.1)?.0)) < 1e-5 * scale {
        // the same point to within a writer's tolerance (an edge's curve ending a hair from its
        // vertex), or apart in (u, v) where the surface's parameters run off (a closed B-spline
        // not quite periodic): joined straight, which moves the measures by about the gap
        out.push(Path::straight(a, b));
        return Ok(());
    }
    let pair = |p: Uv| format!("({}, {})", float_repr(p.0), float_repr(p.1));
    unsupported(format!(
        "a loop that does not join up on its surface ({} to {})",
        pair(a),
        pair(b)
    ))
}
