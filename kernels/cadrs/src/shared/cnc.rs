//! What the `cnc/` cases share: a groove milled round a closed path, as cadrs makes one.
//!
//! An upright cutter, a solid of revolution, run all the way round a closed path at one depth
//! fills at each height the region within its radius there of the path: its section through its
//! axis square to the path, carried round it upright (cadrs's Sweep along the path, the
//! section's plane kept holding the z axis). The path closing, the cutter's two halves beyond a
//! path's ends are within the groove already.
use crate::bench::{arc, p2, p3, unit, v3, P2};
use crate::Refusal;
use cadrs_kernel::{BodyId, BodyKind, Curve2, DefaultKernel, Kernel, PathCurve, Plane, Profile, SweepControl, SweepSpec};

/// The solid `section` sweeps carried upright round the closed `path` in the plane z = 0: the
/// section drawn in the plane through the path's start square to it, `across` the way to the
/// path's left there (its sketch x), its sketch y up.
pub fn round_the_path(k: &mut DefaultKernel, section: &Profile, path: Vec<Curve2>) -> Result<BodyId, Refusal> {
    let path: Vec<PathCurve> = path.into_iter().map(|c| PathCurve::Sketch { plane: Plane::xy(0.0), curve: c }).collect();
    let spec = SweepSpec {
        body: BodyKind::Solid,
        path,
        control: SweepControl::LockDirection(unit(v3(0.0, 0.0, 1.0))),
        corners: Default::default(),
        faces: vec![],
    };
    Ok(k.sweep_with(section, &spec)?.first()?)
}

/// The plane a cutter's section is drawn in where a path starts at `(x, y)` heading along
/// `ahead` (in the plane z = 0): through the point, square to the path, sketch x to the path's
/// left, sketch y up.
pub fn across(x: f64, y: f64, ahead: (f64, f64)) -> Plane {
    let left = v3(-ahead.1, ahead.0, 0.0);
    Plane::new(p3(x, y, 0.0), left, left.cross(&v3(0.0, 0.0, 1.0)))
}

/// The closed path round the polygon `corners` (anticlockwise), every corner rounded to `radius`
/// by an arc tangent to both its sides: from the middle of the first side round and back to it.
pub fn rounded_loop(corners: &[(f64, f64)], radius: f64) -> Vec<Curve2> {
    let n = corners.len();
    let at = |k: usize| corners[k % n];
    let dir = |k: usize| {
        let ((ax, ay), (bx, by)) = (at(k), at(k + 1));
        let len = (bx - ax).hypot(by - ay);
        ((bx - ax) / len, (by - ay) / len)
    };
    let p = |q: (f64, f64)| -> P2 { p2(q.0, q.1) };
    let mut path = Vec::new();
    let middle = ((at(0).0 + at(1).0) / 2.0, (at(0).1 + at(1).1) / 2.0);
    let mut from = middle;
    for k in 1..=n {
        // The corner between side k − 1 and side k: where its arc leaves the one and joins the
        // other, and which way it turns.
        let (c, a, b) = (at(k), dir(k - 1), dir(k));
        let turn = a.0 * b.1 - a.1 * b.0;
        let (leave, join) = ((c.0 - a.0 * radius, c.1 - a.1 * radius), (c.0 + b.0 * radius, c.1 + b.1 * radius));
        // (Its centre: `radius` to the left of the side for a left turn, to the right for a right.)
        let side = turn.signum();
        let centre = (leave.0 - a.1 * radius * side, leave.1 + a.0 * radius * side);
        path.push(Curve2::line(p(from), p(leave)));
        let start = (leave.1 - centre.1).atan2(leave.0 - centre.0).to_degrees();
        path.push(arc(p(centre), radius, start, 90.0 * side));
        from = join;
    }
    path.push(Curve2::line(p(from), p(middle)));
    path
}

/// A program as it runs: the workpiece so far, the cutter (drawn upright with its tip at the
/// origin) and where its tip is. Each move that reaches the workpiece is cut from it there and
/// then: cadrs's cut by a moving solid, a slide or a turn about an upright line.
pub struct Program {
    pub body: BodyId,
    cutter: BodyId,
    /// The cutter's radius, and the box the workpiece fills.
    radius: f64,
    block: (crate::bench::P3, crate::bench::P3),
    tip: crate::bench::P3,
    /// What the moves since the last [`Program::cut`] swept, where they are gathered to be cut
    /// together ([`Program::gathering`]).
    swept: Option<Vec<BodyId>>,
}

impl Program {
    /// `cutter` (of `radius`, upright, its tip at the origin) standing with its tip at `tip`
    /// over the workpiece `body`, which fills the box from `min` to `max`.
    pub fn new(body: BodyId, block: (crate::bench::P3, crate::bench::P3), cutter: BodyId, radius: f64, tip: crate::bench::P3) -> Self {
        Program { body, cutter, radius, block, tip, swept: None }
    }

    /// From here on each straight move's sweep is kept (cadrs's sweep of a solid, one slide)
    /// until [`Program::cut`] takes them all from the workpiece in one boolean: a pass of many
    /// short moves, each overlapping the next.
    pub fn gathering(mut self) -> Self {
        self.swept = Some(Vec::new());
        self
    }

    /// The gathered sweeps cut from the workpiece, all at once.
    pub fn cut(&mut self, k: &mut DefaultKernel) -> Result<(), Refusal> {
        if let Some(tools) = self.swept.as_mut().filter(|t| !t.is_empty()) {
            self.body = crate::bench::boolean(k, cadrs_kernel::BoolOp::Subtract, self.body, tools)?;
            tools.clear();
        }
        Ok(())
    }

    /// The same workpiece under another cutter, its tip at `tip`.
    pub fn change(&mut self, cutter: BodyId, radius: f64, tip: crate::bench::P3) {
        (self.cutter, self.radius, self.tip) = (cutter, radius, tip);
    }

    /// Whether the cutter, its tip anywhere in the box from `a` to `b`, may touch the workpiece:
    /// not while its tip is above it, or it stands wholly beside it.
    fn reaches(&self, a: crate::bench::P3, b: crate::bench::P3) -> bool {
        let (min, max) = self.block;
        let r = self.radius;
        a.z.min(b.z) < max.z
            && a.x.max(b.x) + r > min.x
            && a.x.min(b.x) - r < max.x
            && a.y.max(b.y) + r > min.y
            && a.y.min(b.y) - r < max.y
    }

    /// A straight move of the tip to `to`, the cutter only sliding.
    pub fn line(&mut self, k: &mut DefaultKernel, to: (f64, f64, f64)) -> Result<(), Refusal> {
        let to = p3(to.0, to.1, to.2);
        let v = to - self.tip;
        if v.norm() > 1e-12 && self.reaches(self.tip, to) {
            let placed = crate::bench::moved(k, self.cutter, self.tip.coords)?;
            match self.swept.as_mut() {
                Some(tools) => tools.push(crate::shared::swept::slid(k, placed, v)?),
                None => {
                    self.body =
                        crate::shared::swept::cut_swept(k, self.body, placed, &crate::shared::swept::slide(v))?
                }
            }
        }
        self.tip = to;
        Ok(())
    }

    /// A move of the tip round the upright line through `centre` by `degrees` (anticlockwise
    /// seen from above when positive), at its own height.
    pub fn arc(&mut self, k: &mut DefaultKernel, centre: (f64, f64), degrees: f64) -> Result<(), Refusal> {
        let (dx, dy) = (self.tip.x - centre.0, self.tip.y - centre.1);
        let r = dx.hypot(dy);
        let a = degrees.to_radians();
        let to = p3(centre.0 + dx * a.cos() - dy * a.sin(), centre.1 + dx * a.sin() + dy * a.cos(), self.tip.z);
        // (The arc's own box, generously: the circle's.)
        let (lo, hi) = (p3(centre.0 - r, centre.1 - r, self.tip.z), p3(centre.0 + r, centre.1 + r, self.tip.z));
        if self.reaches(lo, hi) {
            let placed = crate::bench::moved(k, self.cutter, self.tip.coords)?;
            let about = cadrs_kernel::Axis::new(p3(centre.0, centre.1, 0.0), v3(0.0, 0.0, 1.0));
            self.body = crate::shared::swept::cut_swept(k, self.body, placed, &crate::shared::swept::swing(about, 0.0, degrees))?;
        }
        self.tip = to;
        Ok(())
    }

    /// The moves of [`rounded_loop`] round `corners` at the tip's height, from the middle of the
    /// first side (where the tip is) and back to it.
    pub fn rounded_loop(&mut self, k: &mut DefaultKernel, corners: &[(f64, f64)], radius: f64) -> Result<(), Refusal> {
        let z = self.tip.z;
        for c in rounded_loop(corners, radius) {
            match c {
                Curve2::Arc { center, sweep, .. } => self.arc(k, (center.x, center.y), sweep.to_degrees())?,
                other => {
                    let end = other.end();
                    self.line(k, (end.x, end.y, z))?
                }
            }
        }
        Ok(())
    }
}
