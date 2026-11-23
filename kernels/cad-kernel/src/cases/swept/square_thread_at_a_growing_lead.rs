//! A rod, the cylinder of radius 5 about z from z = 0 to 20, threaded by the bar 4 ≤ x ≤ 12,
//! −8 ≤ y ≤ 8, −1 ≤ z ≤ 1 carried so that, turned by θ about z (right-handed), it has risen
//! 6s + 0.3s², s = θ/360°, θ from −90° to 1260°: a lead growing from about 6 to about 8.
//!
//! A screw whose lead grows is no constant twist, so `swept::closed::cut_from` takes the contact's
//! road under any motion (`swept::general`): each face's contact a chord where its normal is square
//! to the motion's velocity, walls between them, trimmed by membership and cut from the rod.
use crate::sketch::block;
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::tolerance;
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Pace, Screw, Trajectory};
use cad_kernel::util::interval::{Error as IntervalError, Interval};
use cad_kernel::KernelError;
use std::f64::consts::TAU;
use std::sync::Arc;

const ROD_RADIUS: f64 = 5.;
const ROD_LENGTH: f64 = 20.;
/// The rise's terms: `LEAD·s + GROWTH·s²` after `s` turns.
const LEAD: f64 = 6.;
const GROWTH: f64 = 0.3;
const ROLL: [f64; 2] = [-90f64.to_radians(), 1260f64.to_radians()];

/// How far the bar has risen when it has turned by θ: `LEAD·s + GROWTH·s²`, `s = θ/τ`, and its
/// rate by θ.
#[derive(Debug)]
struct Rise;

impl Rise {
    fn of(s: f64) -> f64 { LEAD * s + GROWTH * s * s }
}

impl Pace for Rise {
    fn jet(&self, theta: f64) -> Result<[f64; 2], KernelError> {
        let s = theta / TAU;
        Ok([Rise::of(s), (LEAD + 2. * GROWTH * s) / TAU])
    }
    /// The parabola's least and greatest over the stretch: at its ends, or at its vertex where the
    /// stretch holds it.
    fn range(&self, [a, b]: [f64; 2]) -> Result<[f64; 2], KernelError> {
        let (s0, s1) = (a.min(b) / TAU, a.max(b) / TAU);
        let vertex = -LEAD / (2. * GROWTH);
        let mut values = vec![Rise::of(s0), Rise::of(s1)];
        if s0 < vertex && vertex < s1 { values.push(Rise::of(vertex)); }
        Ok([values.iter().cloned().fold(f64::INFINITY, f64::min),
            values.iter().cloned().fold(f64::NEG_INFINITY, f64::max)])
    }
    /// The rise and its rate over a stretch of θ, in interval arithmetic: `s·(LEAD + GROWTH·s)` and
    /// `(LEAD + 2·GROWTH·s)/τ`, `s = θ/τ` (what the sweep's candidates' zones are read by).
    fn enclose(&self, roll: Interval) -> Result<[Interval; 2], IntervalError> {
        let k = |x: f64| Interval::point(x);
        let s = roll.div(k(TAU)?)?;
        let rise = s.mul(k(LEAD)?.add(k(GROWTH)?.mul(s)?)?)?;
        let rate = k(LEAD)?.add(k(2. * GROWTH)?.mul(s)?)?.div(k(TAU)?)?;
        Ok([rise, rate])
    }
}

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let rod = prism(&disc([0., 0., 0.], up, ROD_RADIUS), 0., ROD_LENGTH)?;
    let bar = block([4., -8., -1.], [12., 8., 1.])?;
    // as a lathe cuts it: the tool slides up z at the growing rise (a slide rising a unit a unit
    // of its own parameter, taken at the rise's pace), seen from the rod, which turns by −θ under
    // it (`Relative`: the source's pose, then the observer's undone, so the bar turns by θ)
    let rise = Trajectory::Twist(Screw { origin: [0., 0., 0.], axis: up, ratio: 0., advance: TAU });
    let tool = Trajectory::Paced { path: Box::new(rise), pace: Arc::new(Rise) };
    let spindle = Trajectory::Twist(Screw {
        origin: [0., 0., 0.], axis: up, ratio: -1., advance: 0.,
    });
    let path = Trajectory::Relative { source: Box::new(tool), observer: Box::new(spindle) };
    // no constant twist and no path of slides and turns: the kernel's road for any other rigid
    // motion (K2)
    let size = rod.size().max(bar.size());
    cut_from(&rod, &bar, &path, ROLL, bar.size(), tolerance(size))
        .map_err(|why| format!("the bar's sweep not built: {why:?}"))
}
