//! Integration: a face's contribution to every measure.
use crate::geom::*;
use crate::loops::*;
use crate::py::*;
use crate::surface::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

pub type Measures = [f64; NM];

/// A face's measures, or why its loops bound nothing that can be measured (said of the solid
/// only if this reading of the face is the one asked for).
pub type Settled = Result<Measures, String>;

pub struct Face<'a> {
    /// Which of the solid's faces it is, and the first of them found unmeasurable so far: a
    /// face after that one is not asked for (Python stops there), and gives up.
    index: usize,
    failed: &'a AtomicUsize,
    s0: Arc<Surface>,
    ss: bool,
    c: V3,
    l: f64,
    s: View,
    vref: f64,
    top: Option<f64>,
    /// Whether a loop runs along a seam (one edge both ways), which lies inside its face.
    seam: bool,
    /// The inner integral's breaks.
    cuts: Vec<f64>,
    /// Gauss's nodes and weights over each whole panel between two breaks, once made.
    whole: Vec<Option<(Vec<f64>, Vec<f64>)>>,
    column: Option<Measures>,
    nodes: Vec<f64>,
    weights: Vec<f64>,
    points: Vec<[f64; 6]>,
}

const NOT_CONVERGING: &str =
    "the integral round a loop does not converge (a surface that folds or cusps there?)";

/// Why a face is no face, as the standard reads its loops: they run clockwise about its normal,
/// on a surface they do not close round.
pub const UNBOUNDED: &str = "a face whose loops bound no region of its surface";

fn bisect_right(a: &[f64], x: f64) -> usize {
    let (mut lo, mut hi) = (0, a.len());
    while lo < hi {
        let mid = (lo + hi) / 2;
        if x < a[mid] {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

fn bisect_left(a: &[f64], x: f64) -> usize {
    let (mut lo, mut hi) = (0, a.len());
    while lo < hi {
        let mid = (lo + hi) / 2;
        if a[mid] < x {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

/// Gauss's nodes and weights over [x, y] of the inner parameter (in quarter turns, along an
/// angle), added to `nodes` and `weights`.
fn panel(
    x: f64,
    y: f64,
    rule: &(Vec<f64>, Vec<f64>),
    angle: bool,
    nodes: &mut Vec<f64>,
    weights: &mut Vec<f64>,
) -> R<()> {
    let n = if angle {
        let turns = (y - x) / (PI / 2.0);
        pymax(1.0, integer(turns, turns.ceil())?)
    } else {
        1.0
    };
    let mut k = 0.0;
    while k < n {
        let (a, b) = (x + (y - x) * k / n, x + (y - x) * (k + 1.0) / n);
        let (h, m) = ((b - a) / 2.0, (b + a) / 2.0);
        nodes.extend(rule.0.iter().map(|xi| m + h * xi));
        weights.extend(rule.1.iter().map(|wi| h * wi));
        k += 1.0;
    }
    Ok(())
}

impl<'a> Face<'a> {
    /// The face's contribution to every measure (NM of them), its normal outward where the
    /// face's own is: as the standard reads its loops, and, where that differs, as a writer who
    /// directs them by the surface's normal would mean it.
    pub fn measure(
        s0: Arc<Surface>,
        loops: Vec<Loop>,
        same_sense: bool,
        centre: V3,
        scale: f64,
        index: usize,
        failed: &'a AtomicUsize,
    ) -> R<(Settled, Option<Settled>, Option<Settled>)> {
        let (mut face, paths) = Face::laid(s0, loops, same_sense, centre, scale, index, failed)?;
        let transposed = face.s.transposed;
        let bounded = !paths.is_empty();
        let periods = face.s.periods();
        let singular = face.s.singular();
        let mut total = [0.0; NM];
        for ps in &paths {
            for p in ps {
                let part = face.green(p)?;
                for i in 0..NM {
                    total[i] += part[i];
                }
            }
        }
        if !bounded && face.s0.not_mirrored() {
            if let Some(per) = periods[1] {
                let mut turns = Vec::new();
                for x in singular.iter().filter(|x| x.axis == 1) {
                    turns.push(round9(pymod(x.val, per)?));
                }
                if sorted_set(turns).len() > 1 {
                    // bounded by points alone, on a surface whose closed profile meets its axis
                    // twice: the face could be either part of the profile turned
                    return unsupported(
                        "a face bounded by points alone, on a surface whose closed profile \
                         meets its axis twice: which part of it is the face is not said",
                    );
                }
            }
        }
        let sigma = sense(face.ss, transposed);
        let tau = sense(true, transposed);
        // the face as the standard reads its loops (by its own normal, sigma), and, where that
        // differs, as a writer who directs them by the surface's normal would mean it (tau)
        let standard = face.settle(&total, sigma, sigma, false)?;
        let other = if tau != sigma {
            Some(face.settle(&total, tau, sigma, false)?)
        } else {
            None
        };
        // and, where a loop along a seam runs against the standard, as its writer means it: the
        // region the loop unwraps to, which the seam is in
        let seamed = if face.seam {
            Some(face.settle(&total, sigma, sigma, true)?).filter(|x| *x != standard)
        } else {
            None
        };
        Ok((standard, other, seamed))
    }

    /// Whether the face's loops, as the standard reads them, plainly bound no region of its
    /// surface (`UNBOUNDED`): what `measure` would find of it, told from the loops alone, by the
    /// area in (u, v) of the polygons through their paths' samples, -∮ (v - v_ref) du. It takes
    /// no integral and sets nothing up to take one, so it is known of every face at once (the
    /// module's own comment says why that is wanted). The polygons' area is taken twice, through
    /// every sample and through every other one, and the two extrapolated (the area is off as
    /// the square of the samples' spacing); the face is said to bound nothing only where its
    /// area is against the standard by far more than the two differ, and every sample lies on
    /// the surface where its (u, v) says. Any other face is left to `measure`.
    pub fn unbounded(s0: &Arc<Surface>, loops: &[Loop], same_sense: bool, scale: f64) -> R<bool> {
        let (s, vref, top) = lie(s0, loops)?;
        if turn(&s, vref, top).is_some() {
            // what its loops leave open is made up in whole turns of the surface (`settle`):
            // its area comes to more than nothing whichever way they run
            return Ok(false);
        }
        let at = |q: Uv| if s.transposed { (q.1, q.0) } else { q };
        let strip = |a: Uv, b: Uv| -((a.1 + b.1) / 2.0 - vref) * (b.0 - a.0);
        // (the area, and its pieces each taken as positive: the size it is held against)
        let (mut fine, mut coarse, mut gross) = (0.0, 0.0, 0.0);
        let mut along: Vec<Uv> = Vec::new();
        for p in loops.iter().flat_map(|(ps, _)| ps) {
            along.clear();
            along.extend(p.points().map(at));
            for w in along.windows(2) {
                fine += strip(w[0], w[1]);
                gross += strip(w[0], w[1]).abs();
            }
            let last = along.len().saturating_sub(1);
            let mut k = 0;
            while k < last {
                let next = (k + 2).min(last);
                coarse += strip(along[k], along[next]);
                k = next;
            }
        }
        let area = fine + (fine - coarse) / 3.0;
        // (and by more than `settle` takes for a face of no area)
        if !(sense(same_sense, s.transposed) * area < -(4.0 * (fine - coarse).abs() + 1e-6 * gross)
            && area.abs() > 1e-9 * pymax(1.0, scale * scale))
        {
            return Ok(false);
        }
        // a path that is not where its edge is tells nothing: looked at last, since it takes the
        // surface and the curve at every sample, and nearly every face has been let go by now
        for p in loops.iter().flat_map(|(ps, _)| ps) {
            let Some(c) = &p.c else { continue };
            for (t, uv) in &p.rf {
                if norm(sub(s0.ev(uv.0, uv.1)?.0, c.point(*t)?)) > 1e-6 * scale {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    /// The face set up to be integrated over, and its loops' paths on the (u, v) plane the
    /// inner integral runs along v of (`lie`).
    fn laid(
        s0: Arc<Surface>,
        mut loops: Vec<Loop>,
        same_sense: bool,
        centre: V3,
        scale: f64,
        index: usize,
        failed: &'a AtomicUsize,
    ) -> R<(Face<'a>, Vec<Vec<Path>>)> {
        let (s, vref, top) = lie(&s0, &loops)?;
        if s.transposed {
            for (ps, _) in &mut loops {
                for p in ps {
                    p.transpose();
                }
            }
        }
        let paths: Vec<Vec<Path>> = loops.into_iter().map(|(ps, _)| ps).collect();
        // a seam: an edge run both ways in one loop, its two runs apart (a turn apart) in (u, v)
        let seam = paths.iter().any(|ps| {
            ps.iter().enumerate().any(|(i, p)| {
                ps[i + 1..].iter().any(|q| match (&p.c, &q.c) {
                    (Some(a), Some(b)) => {
                        let (x, y) = (p.start(), q.end());
                        Arc::ptr_eq(a, b)
                            && (p.t0, p.t1) == (q.t1, q.t0)
                            && (x.0 - y.0).abs() + (x.1 - y.1).abs() > 1e-7
                    }
                    _ => false,
                })
            })
        });
        let singular = s.singular();
        let mut face = Face {
            index,
            failed,
            s0,
            ss: same_sense,
            c: centre,
            l: scale,
            s,
            vref,
            top,
            seam,
            cuts: Vec::new(),
            whole: Vec::new(),
            column: None,
            nodes: Vec::new(),
            weights: Vec::new(),
            points: Vec::new(),
        };
        face.layout(&singular);
        Ok((face, paths))
    }

    /// The face's measures from its loops' integrals (`raw`), the loops taken in the sense
    /// `rho`, the face's normal `sigma`; `by_seam`, a loop along a seam taken whichever way it
    /// runs.
    fn settle(&mut self, raw: &Measures, rho: f64, sigma: f64, by_seam: bool) -> R<Settled> {
        let mut total = raw.map(|x| rho * x);
        // what the loops leave open on a periodic surface: whole turns of the inner integral
        if let Some(top) = turn(&self.s, self.vref, self.top) {
            // the whole surface's area in (u, v), signed as its column integral is; the
            // face's must come to (0, |full|]
            let full = self.s.periods()[0].unwrap() * (top - self.vref);
            let a = full.abs();
            let turns = div(total[0], a)? - 1e-12;
            let k = -integer(turns, turns.floor())? * (if full > 0.0 { 1.0 } else { -1.0 });
            if k > 0.0 && self.seam && by_seam {
                // the loop runs against the standard: not the rest of the surface (which
                // the seam is not in), but the region it unwraps to
                total = total.map(|x| -x);
            } else if k != 0.0 {
                let column = match self.column {
                    Some(c) => c,
                    None => {
                        let c = self.whole_turn(top)?;
                        self.column = Some(c);
                        c
                    }
                };
                for i in 0..NM {
                    total[i] += k * column[i];
                }
            }
        }
        let l = self.l;
        if total[0].abs() <= 1e-12 * pymax(1.0, l * l) && total[1].abs() <= 1e-12 * l.powf(2.0) {
            return Ok(Ok([0.0; NM])); // a face of no area (a loop out along an edge and back): nothing
        }
        if total[0] <= 0.0 {
            return Ok(Err(UNBOUNDED.into()));
        }
        // the linear measures by the face's outward normal; its areas are positive
        let mut out = total.map(|x| x * sigma);
        out[0] = total[0];
        out[1] = total[1];
        Ok(Ok(out))
    }

    /// The inner integral's breaks, once a face: the surface's knots and its poles (where the
    /// normal's length has a kink), sorted. Round a periodic parameter they come again every
    /// turn: a loop is followed from where its first edge lands, which on a closed B-spline
    /// surface may be a turn off its knots' own range.
    fn layout(&mut self, singular: &[Sing]) {
        let per = self.s.periods()[1];
        let mut poles: Vec<f64> = singular
            .iter()
            .filter(|x| x.axis == 1)
            .map(|x| x.val)
            .collect();
        let mut knots: Vec<f64> = self.s.breaks(1).to_vec();
        if let Some(per) = per {
            poles = poles
                .iter()
                .flat_map(|v| (-3..4).map(move |k| v + k as f64 * per))
                .collect();
            let lo = self.s.domain()[1].0;
            if lo.is_finite() {
                knots.push(lo); // where one turn joins the next
            }
            knots = knots
                .iter()
                .flat_map(|v| (-3..4).map(move |k| v + k as f64 * per))
                .collect();
        }
        knots.extend(poles);
        self.cuts = sorted_set(knots);
        self.whole = vec![None; self.cuts.len()];
        self.column = None;
    }

    /// f ∫ F(u, s) ds from v_ref to v.
    fn h(&mut self, u: f64, v: f64, f: f64) -> R<Measures> {
        let vref = self.vref;
        if (v - vref).abs() <= 1e-14 * (1.0 + vref.abs()) {
            return Ok([0.0; NM]);
        }
        let (a, b) = if v > vref { (vref, v) } else { (v, vref) };
        let f = if v < vref { -f } else { f };
        let (i, j) = (bisect_right(&self.cuts, a), bisect_left(&self.cuts, b));
        self.nodes.clear();
        self.weights.clear();
        let rule = gauss(self.s.nodes()[1]);
        let angle = self.s.angles()[1];
        if i < j {
            panel(
                a,
                self.cuts[i],
                &rule,
                angle,
                &mut self.nodes,
                &mut self.weights,
            )?;
            for k in i..j - 1 {
                // a whole panel, which every column shares
                if self.whole[k].is_none() {
                    let (mut ns, mut ws) = (Vec::new(), Vec::new());
                    panel(
                        self.cuts[k],
                        self.cuts[k + 1],
                        &rule,
                        angle,
                        &mut ns,
                        &mut ws,
                    )?;
                    self.whole[k] = Some((ns, ws));
                }
                let (ns, ws) = self.whole[k].as_ref().unwrap();
                self.nodes.extend_from_slice(ns);
                self.weights.extend_from_slice(ws);
            }
            panel(
                self.cuts[j - 1],
                b,
                &rule,
                angle,
                &mut self.nodes,
                &mut self.weights,
            )?;
        } else {
            panel(a, b, &rule, angle, &mut self.nodes, &mut self.weights)?;
        }
        if f != 1.0 {
            for w in &mut self.weights {
                *w = f * *w;
            }
        }
        self.s.col(u, &self.nodes, &mut self.points)?;
        let [cx, cy, cz] = self.c;
        let mut acc = [0.0f64; NM];
        for (p, &w) in self.points.iter().zip(&self.weights) {
            let [px, py, pz, nx, ny, nz] = *p;
            let (x, y, z) = (px - cx, py - cy, pz - cz);
            let (xx, yy, zz) = (x * x, y * y, z * z);
            let (wx, wy, wz) = (w * nx, w * ny, w * nz);
            acc[0] += w;
            acc[1] += w * (nx * nx + ny * ny + nz * nz).sqrt();
            acc[2] += x * wx + y * wy + z * wz;
            acc[3] += xx * wx;
            acc[4] += yy * wy;
            acc[5] += zz * wz;
            acc[6] += xx * x * wx;
            acc[7] += yy * y * wy;
            acc[8] += zz * z * wz;
            acc[9] += xx * y * wx;
            acc[10] += yy * z * wy;
            acc[11] += zz * x * wz;
        }
        Ok([
            acc[0],
            acc[1],
            acc[2] / 3.0,
            acc[3] / 2.0,
            acc[4] / 2.0,
            acc[5] / 2.0,
            acc[6] / 3.0,
            acc[7] / 3.0,
            acc[8] / 3.0,
            acc[9] / 2.0,
            acc[10] / 2.0,
            acc[11] / 2.0,
        ])
    }

    /// ∫ H(u, top) du over a whole turn of u: the whole surface, between v_ref and top.
    fn whole_turn(&mut self, top: f64) -> R<Measures> {
        let pu = self.s.periods()[0].unwrap();
        let line = Path::straight((0.0, top), (pu, top));
        // -∮ H du along the top, traversed backwards
        Ok(self.green(&line)?.map(|x| -x))
    }

    /// One more point of a path's integral, which may take BUDGET of them and no more.
    fn spend(&mut self, spent: &mut usize) -> R<()> {
        *spent += 1;
        if *spent > BUDGET {
            return unsupported(NOT_CONVERGING);
        }
        if self.failed.load(Ordering::Relaxed) < self.index {
            return unsupported("a face before this one could not be measured");
        }
        Ok(())
    }

    /// The integrand of the integral round a loop at `t` of a path; and the path's rate du/dt
    /// there, and how square the surface's rates stand (`Path::uv_square`).
    fn f(&mut self, path: &Path, t: f64, spent: &mut usize) -> R<(Measures, f64, f64)> {
        self.spend(spent)?;
        let (u, v, du, _, square) = path.uv_square(&self.s, t)?;
        Ok((self.h(u, v, -du)?, du, square))
    }

    /// Gauss-Kronrod's estimate over [a, b], halved until it is near enough.
    ///
    /// That estimate is of ∫ H u'(t) dt, and u'(t) is the curve's direction resolved along the
    /// surface's two rates: the rate of the path the curve would trace in (u, v) if it lay in
    /// the surface. H is read on another path, the curve's points inverted onto the surface,
    /// which is the one that closes the loop. An edge's curve stands a writer's tolerance off
    /// its surface, and the two paths part by about that; by far more by a corner where one of
    /// the surface's rates dies away without the surface saying it has a pole, or along an edge
    /// of its domain that the curve stands just outside. So the estimate, once near enough, is
    /// made the integral along the inverted path: by how far the two paths part over the panel
    /// (`parting`), times H, the mean of its values at the panel's ends, where that is more
    /// than the panel's tolerance; and the panel is halved first where half H's change over
    /// it, times the parting, which is all the mean can be off by, is more than a sixty-fourth
    /// of that (a path may be many such panels, and Gauss-Kronrod's rule, once near enough, is
    /// far nearer than it is asked to be).
    ///
    /// Where the surface's rates run together at any of the rule's points (FOLDED: a fold of
    /// an envelope, which a face may be trimmed along) there is no such estimate to mend. The
    /// curve's direction resolved along the rates is the quotient of two roundings there, and
    /// the inverted points land the tolerance over the sine of the rates' angle along the fold
    /// from where any direction says: the panel is taken by `chords`, which follows the
    /// inverted path alone.
    fn adaptive(
        &mut self,
        path: &Path,
        a: f64,
        b: f64,
        depth: usize,
        tol: &Measures,
        spent: &mut usize,
    ) -> R<Measures> {
        let (hw, mid) = ((b - a) / 2.0, (b + a) / 2.0);
        let mut k = [0.0; NM];
        let mut g = [0.0; NM];
        let (mut went, mut square) = (0.0, 1.0);
        for &(x, wk, wg) in kronrod() {
            let (y, du, here) = self.f(path, mid + hw * x, spent)?;
            for i in 0..NM {
                k[i] += wk * y[i];
                g[i] += wg * y[i];
            }
            went += wk * du;
            square = pymin(square, here);
        }
        let mut halve = depth < 24
            && (0..NM).any(|i| (hw * (k[i] - g[i])).abs() > tol[i] + 1e-12 * (hw * k[i]).abs());
        let mut mend = None;
        if square < FOLDED {
            match self.chords(path, a, b, hw * went, tol, spent)? {
                Some(out) => return Ok(out),
                None if depth < 48 => halve = true,
                None => return unsupported(NOT_CONVERGING),
            }
        } else if !halve {
            if let Some((parted, ha, hb)) = self.parting(path, a, b, hw * went, tol, spent)? {
                halve = (0..NM).any(|i| (0.5 * parted * (hb[i] - ha[i])).abs() > tol[i] / 64.0);
                if halve && depth >= 48 {
                    // halved as far as parameters go and still parted: the inverted path
                    // breaks here. Where its two sides are one point of the surface it is
                    // joined, as a gap between two paths is (loops.rs, `gap`); else the loop
                    // is not laid on the surface at all
                    let (pa, pb) = (self.on(path, a)?, self.on(path, b)?);
                    let (sa, sb) = (self.s.ev(pa.0, pa.1)?.0, self.s.ev(pb.0, pb.1)?.0);
                    if norm(sub(sa, sb)) > 1e-5 * self.l {
                        return unsupported(
                            "a loop whose path on its surface breaks (an edge laid across a \
                             place where its surface meets itself?)",
                        );
                    }
                    halve = false;
                }
                mend = Some((parted, ha, hb));
            }
        }
        if halve {
            let l = self.adaptive(path, a, mid, depth + 1, tol, spent)?;
            let r = self.adaptive(path, mid, b, depth + 1, tol, spent)?;
            let mut out = [0.0; NM];
            for i in 0..NM {
                out[i] = l[i] + r[i];
            }
            return Ok(out);
        }
        let mut out = k.map(|x| hw * x);
        if let Some((parted, ha, hb)) = mend {
            for i in 0..NM {
                out[i] -= 0.5 * (ha[i] + hb[i]) * parted;
            }
        }
        Ok(out)
    }

    /// How far the path the integrand's rate follows and the path its H is read on part over
    /// [a, b], and H at the inverted path's two ends there; None where that is too little to
    /// count, by `tol`. `went` is the rate integrated over [a, b]; the parting is how far the
    /// inverted path went in u, less that.
    fn parting(
        &mut self,
        path: &Path,
        a: f64,
        b: f64,
        went: f64,
        tol: &Measures,
        spent: &mut usize,
    ) -> R<Option<(f64, Measures, Measures)>> {
        if path.line.is_some() {
            return Ok(None);
        }
        let (pa, pb) = (self.on(path, a)?, self.on(path, b)?);
        // (round a period the inverted path may come a whole turn off, where its reference
        // path is too coarse to say which turn: H is the same there, and the paths have not
        // parted)
        let parted = nearest(pb.0 - pa.0 - went, 0.0, self.s.periods()[0])?;
        // rounding's, on any path, or a writer's tolerance's: no H is read for it
        if parted.abs() <= 1e-12 * (1.0 + pymax(pa.0.abs(), pb.0.abs()))
            || parted.abs() <= PARTED * went.abs()
        {
            return Ok(None);
        }
        // or rounding's by a pole, where u is an angle about a point the path comes near, and
        // inverting a point there gives its angle no better than the point's own rounding over
        // how far it is from the pole (|S_u|): it is the rate that is right there
        let [cx, cy, cz] = self.c;
        let reach = self.l + pymax(pymax(cx.abs(), cy.abs()), cz.abs());
        for p in [pa, pb] {
            let (_, su, _) = self.s.ev(p.0, p.1)?;
            if parted.abs() * norm(su) <= 1e3 * f64::EPSILON * reach {
                return Ok(None);
            }
        }
        self.spend(spent)?;
        let ha = self.h(pa.0, pa.1, 1.0)?;
        self.spend(spent)?;
        let hb = self.h(pb.0, pb.1, 1.0)?;
        if (0..NM).all(|i| (parted * pymax(ha[i].abs(), hb[i].abs())).abs() <= tol[i]) {
            return Ok(None);
        }
        Ok(Some((parted, ha, hb)))
    }

    /// Where a path is at `t`, on the (u, v) plane: its curve's point there inverted onto the
    /// surface, and at its ends where the loop's next piece takes it up.
    fn on(&self, path: &Path, t: f64) -> R<Uv> {
        if path.line.is_none() {
            if t == path.t0 {
                return Ok(path.start());
            }
            if t == path.t1 {
                return Ok(path.end());
            }
        }
        let (u, v, _, _) = path.uv(&self.s, t)?;
        Ok((u, v))
    }

    /// No less than the size of each measure's integrand F at any of `at`.
    fn most(&mut self, at: &[Uv]) -> R<Measures> {
        let mut out = [0.0; NM];
        let [cx, cy, cz] = self.c;
        for &(u, v) in at {
            self.s.col(u, &[v], &mut self.points)?;
            let [px, py, pz, nx, ny, nz] = self.points[0];
            let r = (px - cx).hypot(py - cy).hypot(pz - cz);
            let n = nx.hypot(ny).hypot(nz);
            let here = [1.0, n, n * r / 3.0, n * r * r / 2.0, n * r * r * r / 2.0];
            for i in 0..NM {
                out[i] = pymax(out[i], here[[0, 1, 2, 3, 3, 3, 4, 4, 4, 4, 4, 4][i]]);
            }
        }
        Ok(out)
    }

    /// -∫ H du along the straight line from `a` to `b` of the (u, v) plane, by Gauss-Kronrod's
    /// rule over the whole of it; None where its two estimates are apart by more than `tol`.
    fn chord(&mut self, a: Uv, b: Uv, tol: &Measures, spent: &mut usize) -> R<Option<Measures>> {
        let (du, dv) = (b.0 - a.0, b.1 - a.1);
        if du == 0.0 {
            return Ok(Some([0.0; NM]));
        }
        let mut k = [0.0; NM];
        let mut g = [0.0; NM];
        for &(x, wk, wg) in kronrod() {
            self.spend(spent)?;
            let s = 0.5 + 0.5 * x;
            let y = self.h(a.0 + s * du, a.1 + s * dv, -du)?;
            for i in 0..NM {
                k[i] += wk * y[i];
                g[i] += wg * y[i];
            }
        }
        if (0..NM).any(|i| (0.5 * (k[i] - g[i])).abs() > tol[i] + 1e-12 * (0.5 * k[i]).abs()) {
            return Ok(None);
        }
        Ok(Some(k.map(|x| 0.5 * x)))
    }

    /// The integral over [a, b] of a path along two chords of it, from its point at `a` to its
    /// point at the middle to its point at `b`; None where that is not near enough, and [a, b]
    /// is to be halved. `went` is the path's rate integrated over [a, b], which is near enough
    /// how far it goes there to say whether that is any part of a turn.
    ///
    /// The integral asks for no derivative of the path. It is that of the form H du along it,
    /// and by Green's theorem it differs between two paths with the same ends by ∬ F du dv
    /// over what lies between them. So the chords' integral is off by no more than F over the
    /// triangle of the three points: little where the path is nearly straight, and little
    /// wherever the surface's rates run together or one of them dies away, for F is the
    /// surface's normal S_u × S_v, which is nothing there, however far along the fold the
    /// points have landed. The triangle's area times twice the most of F at its corners is
    /// held to a sixty-fourth of `tol` (the chords come no nearer than they are asked to, where
    /// Gauss-Kronrod's rule, once near enough, is far nearer, and a path may be many panels of
    /// them), and each chord's own quadrature to `tol`.
    fn chords(
        &mut self,
        path: &Path,
        a: f64,
        b: f64,
        went: f64,
        tol: &Measures,
        spent: &mut usize,
    ) -> R<Option<Measures>> {
        let mid = (b + a) / 2.0;
        let (pa, mut pm, mut pb) = (self.on(path, a)?, self.on(path, mid)?, self.on(path, b)?);
        // round a period, each point on the turn of the one before: a piece long enough to go
        // half a turn is halved first
        let periods = self.s.periods();
        if let Some(per) = periods[0] {
            if went.abs() > per / 8.0 {
                return Ok(None);
            }
        }
        pm = (
            nearest(pm.0, pa.0, periods[0])?,
            nearest(pm.1, pa.1, periods[1])?,
        );
        pb = (
            nearest(pb.0, pm.0, periods[0])?,
            nearest(pb.1, pm.1, periods[1])?,
        );
        let area = 0.5 * ((pm.0 - pa.0) * (pb.1 - pa.1) - (pm.1 - pa.1) * (pb.0 - pa.0)).abs();
        let most = self.most(&[pa, pm, pb])?;
        if (0..NM).any(|i| 2.0 * area * most[i] > tol[i] / 64.0) {
            return Ok(None);
        }
        let Some(first) = self.chord(pa, pm, tol, spent)? else {
            return Ok(None);
        };
        let Some(second) = self.chord(pm, pb, tol, spent)? else {
            return Ok(None);
        };
        let mut out = [0.0; NM];
        for i in 0..NM {
            out[i] = first[i] + second[i];
        }
        Ok(Some(out))
    }

    /// -∫ H(u(t), v(t)) u'(t) dt along a path: nothing along a line of constant u (a seam, a
    /// meridian, a B-spline face's own boundary), where du is nothing.
    fn green(&mut self, path: &Path) -> R<Measures> {
        let us: Vec<f64> = path.points().map(|uv| uv.0).collect();
        let (mut hi, mut lo, mut big) = (us[0], us[0], us[0].abs());
        for &x in &us[1..] {
            hi = pymax(hi, x);
            lo = pymin(lo, x);
            big = pymax(big, x.abs());
        }
        if hi - lo <= 1e-13 * (1.0 + big) {
            return Ok([0.0; NM]);
        }
        let (t0, t1) = if path.line.is_some() {
            (0.0, 1.0)
        } else {
            (path.t0, path.t1)
        };
        let (tlo, thi) = (pymin(t0, t1), pymax(t0, t1));
        let mut inner: Vec<f64> = match &path.c {
            Some(c) => sorted(
                c.breaks()
                    .iter()
                    .copied()
                    .filter(|b| tlo < *b && *b < thi)
                    .collect(),
            ),
            None => Vec::new(),
        };
        if t1 < t0 {
            inner.reverse();
        }
        let mut cuts = vec![t0];
        cuts.extend(inner);
        cuts.push(t1);
        let l = self.l;
        // each panel's error to TOLERANCE of the solid's size's power: the measures are graded to
        // 1e-6, and some thousand panels' errors still come well short of that
        let e = TOLERANCE;
        let (l4, l5) = (e * l.powf(4.0), e * l.powf(5.0));
        let tol: Measures = [
            1e-9 * pymax(l * l, 1.0),
            e * l.powf(2.0),
            e * l.powf(3.0),
            l4,
            l4,
            l4,
            l5,
            l5,
            l5,
            l5,
            l5,
            l5,
        ];
        let mut spent = 0usize;
        // a curve of very many pieces (a polyline, a B-spline fitted point by point): each piece
        // tiny beside the whole, and smooth, so Gauss's seven points are exact to well within the
        // tolerance there, with no estimate of the error
        // (a kink between pieces is no smoothness an estimate of the error can rely on: every
        // break is a cut; along a straight piece, four points are plenty)
        let small = if cuts.len() > 129 {
            (t1 - t0).abs() / 128.0
        } else {
            0.0
        };
        let straight = path.c.as_ref().is_some_and(|c| c.degree() == Some(1));
        let rule: Vec<(f64, f64)> = if straight {
            let g = gauss(4);
            g.0.iter().copied().zip(g.1.iter().copied()).collect()
        } else {
            kronrod()
                .iter()
                .filter(|k| k.2 != 0.0)
                .map(|k| (k.0, k.2))
                .collect()
        };
        let mut out = [0.0; NM];
        for w in cuts.windows(2) {
            let (a, b) = (w[0], w[1]);
            if a == b {
                continue;
            }
            let part = if (b - a).abs() < small {
                let (hw, mid) = ((b - a) / 2.0, (b + a) / 2.0);
                let mut part = [0.0; NM];
                for &(x, wg) in &rule {
                    let (y, _, _) = self.f(path, mid + hw * x, &mut spent)?;
                    for i in 0..NM {
                        part[i] += hw * wg * y[i];
                    }
                }
                part
            } else {
                self.adaptive(path, a, b, 0, &tol, &mut spent)?
            };
            for i in 0..NM {
                out[i] += part[i];
            }
        }
        Ok(out)
    }
}

/// Where a face lies on its surface for the integral over it: the surface seen so that the inner
/// integral runs along v (its own (u, v), or the two swapped), where that integral starts
/// (v_ref) and the pole it runs to, if any. From the loops as `face_loops` laid them, which
/// are not changed.
fn lie(s0: &Arc<Surface>, loops: &[Loop]) -> R<(View, f64, Option<f64>)> {
    // the inner integral runs along the parameter no loop winds round; where none winds, along
    // the one whose ends are poles (a sphere's or a revolution's meridian), else v
    let poles = s0.singular();
    // free to choose: along the parameter of the surface's poles, else the one of fewer knot
    // spans (each a panel of the inner integral at every point of the outer)
    let spans = s0.spans();
    let prefer_u = (!poles.is_empty() && poles.iter().all(|x| x.axis == 0))
        || (poles.is_empty() && spans[0] < spans[1]);
    let no_u = loops.iter().all(|(_, w)| w.0 == 0.0);
    let no_v = loops.iter().all(|(_, w)| w.1 == 0.0);
    let transposed = if no_v && !(prefer_u && no_u) {
        false
    } else if no_u {
        true
    } else {
        return unsupported("a face whose loops wind round both of its surface's periods");
    };
    let s = View {
        s: s0.clone(),
        transposed,
    };
    // the loops' paths as they lie on that plane: the surface's own, or with u and v swapped
    let at = |q: Uv| if transposed { (q.1, q.0) } else { q };
    let paths = || loops.iter().flat_map(|(ps, _)| ps);
    let bounded = !loops.is_empty();
    let periods = s.periods();
    let domain = s.domain();
    // the inner integral's start: a singular line of the inner parameter, where H is zero, or
    // else where the face is
    // its poles: lines across the inner parameter (and their turns, where it is periodic)
    let singular = s.singular();
    let mut sing: Vec<f64> = singular
        .iter()
        .filter(|x| x.axis == 1)
        .map(|x| x.val)
        .collect();
    if let Some(per) = periods[1] {
        for v in sing.clone() {
            for k in -2..3 {
                sing.push(v + k as f64 * per);
            }
        }
    }
    let sing = sorted_set(sing);
    // a point of the face's boundary, away from any pole
    let first = paths()
        .find(|p| p.line.is_none())
        .map(|p| at(p.rf[p.rf.len() / 2].1))
        .unwrap_or((0.0, 0.0));
    let eps = 1e-12 * (1.0 + first.1.abs());
    let below: Vec<f64> = sing
        .iter()
        .copied()
        .filter(|x| *x < first.1 - eps)
        .collect();
    let above: Vec<f64> = sing
        .iter()
        .copied()
        .filter(|x| *x > first.1 + eps)
        .collect();
    // the inner integral from the pole below the face (H is zero along it), to the pole above
    let vref;
    if !bounded && sing.len() > 1 {
        // bounded by points alone (a whole sphere): from one pole to the next
        // (on a closed profile, the half from its start, as it was drawn)
        let lo0 = if domain[1].0.is_finite() {
            domain[1].0
        } else {
            0.0
        };
        vref = if periods[1].is_none() {
            sing[0]
        } else {
            let mut best: Option<f64> = None;
            for x in sing.iter().copied().filter(|x| *x <= lo0 + 1e-12) {
                best = Some(match best {
                    Some(b) => pymax(b, x),
                    None => x,
                });
            }
            best.unwrap_or(sing[0])
        };
    } else {
        // a pole a loop runs through, where H is zero: near a pole the angle along it is only
        // as good as inversion makes it, and its derivative there is noise, which H must
        // cancel (from the other pole, H there is a whole column, and the noise counts)
        let through: Vec<f64> = sing
            .iter()
            .copied()
            .filter(|x| {
                paths().any(|p| {
                    let close = |q: Uv| (q.1 - x).abs() <= 1e-9 * (1.0 + x.abs());
                    close(at(p.start())) || close(at(p.end()))
                })
            })
            .collect();
        // else the pole nearer the face, so that a small face by a pole is not the difference
        // of the whole surface and most of it
        let (nb, na) = if through.is_empty() {
            (below.clone(), above.clone())
        } else {
            let b: Vec<f64> = through
                .iter()
                .copied()
                .filter(|x| *x < first.1 - eps)
                .collect();
            let a: Vec<f64> = through
                .iter()
                .copied()
                .filter(|x| *x > first.1 + eps)
                .collect();
            (b, a)
        };
        let near = match (nb.last(), na.first()) {
            (Some(b), Some(a)) => Some(if (a - first.1).abs() < (b - first.1).abs() {
                *a
            } else {
                *b
            }),
            (Some(b), None) => Some(*b),
            (None, Some(a)) => Some(*a),
            (None, None) => None,
        };
        vref = near.unwrap_or(first.1);
    }
    let top = if bounded && !below.is_empty() && above.first().is_some_and(|a| *a == vref) {
        // the top: the pole on the face's other side (below it)
        below.last().copied()
    } else {
        sing.iter().copied().find(|x| *x > vref + eps)
    };
    Ok((s, vref, top))
}

/// Where what a face's loops leave open is made up in whole turns of its surface (`lie`'s
/// view of it, v_ref and top): the top of the inner integral. Only a periodic inner parameter, or
/// a singular line across it, leaves a face's loops open: else every side of the face is an edge
/// of it.
fn turn(s: &View, vref: f64, top: Option<f64>) -> Option<f64> {
    let periods = s.periods();
    periods[0]?;
    let top = match (top, periods[1]) {
        (Some(top), _) => top,
        (None, Some(per)) => vref + per,
        (None, None) => return None,
    };
    if top - vref != 0.0 { Some(top) } else { None }
}

/// The sense a face's loops run in on the (u, v) plane of `lie`, as its normal sees them: its
/// surface's turned by `same_sense`, and by the swap of u and v.
fn sense(same_sense: bool, transposed: bool) -> f64 {
    (if same_sense { 1.0 } else { -1.0 }) * (if transposed { -1.0 } else { 1.0 })
}
