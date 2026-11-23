//! A block, the box (−10, −10, −8)–(30, 25, 0), with a groove cut into its top by a bull-nose mill
//! — radius 2 from 1.5 below the top to 5 above it, its bottom corner rounded to 0.5 — whose
//! upright axis rides the cubic Bézier curve (0, 0, 0), (30, 20, 0), (−10, 20, 0), (20, 0, 0) in
//! the top from its first end to its last, its foot on the curve. The curve crosses itself, so the
//! groove does, and turns tighter than the mill.
//!
//! The motion is the mill carried without turning along the curve, the kernel's own B-spline: no
//! slide, turn or screw, nor a path of them, so a motion the kernel reads by evaluating it
//! (`Trajectory::General`, its K2 road), which takes a convex tool of revolution.
//!
//! Refused by the kernel at run time: the sheet each instant's contact sweeps cannot be fitted to
//! the arrangement's tolerance (`Unbuilt::Failed`: it fits only to about 1.9 along the loop and 1.4
//! along the roll), before its fold or its return onto itself is reached. The same refusal stands
//! with the mill carried by the kernel's own frame along the curve (`trajectory::Along`, which
//! turns it only about its own axis, so sweeps the same set); the mill is no prism, so the planar
//! road (`swept::planar_solid`) does not take it.
use crate::sketch::block;
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::geom::{tolerance, Frame, V};
use cad_kernel::nurbs::BSpline;
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Motion, Path, Stretch, Trajectory};
use cad_kernel::util::interval::Interval;
use cad_kernel::util::space::{norm, scale, sub};
use cad_kernel::KernelError;
use std::f64::consts::{FRAC_PI_2, TAU};
use std::sync::Arc;

const STOCK_LO: V = [-10., -10., -8.];
const STOCK_HI: V = [30., 25., 0.];
const CONTROL: [V; 4] = [[0., 0., 0.], [30., 20., 0.], [-10., 20., 0.], [20., 0., 0.]];
/// The mill: the middle of its flat bottom, its radius, its corner's and its top.
const FOOT: V = [0., 0., -1.5];
const RADIUS: f64 = 2.;
const CORNER: f64 = 0.5;
const TOP: f64 = 5.;
/// How closely the mill is meshed for the sweep's membership reads: a two-hundred-and-fiftieth of
/// its corner's radius.
const BAR: f64 = 0.002;

/// The mill about the vertical line through the curve's first point: its meridian in the
/// half-plane y = 0 beyond the axis — out along its flat bottom, round its corner, up its side,
/// back across its top and down its axis — turned once about the axis.
fn mill() -> Result<Brep, String> {
    let at = |x: f64, z: f64| [x, 0., z];
    let [_, _, z0] = FOOT;
    let corner = Frame::new(at(RADIUS - CORNER, z0 + CORNER), [0., -1., 0.], [1., 0., 0.]);
    let edges = vec![
        ProfileEdge::Line { a: at(0., z0), b: at(RADIUS - CORNER, z0) },
        ProfileEdge::Arc { frame: corner, r: CORNER, span: Some([-FRAC_PI_2, 0.]) },
        ProfileEdge::Line { a: at(RADIUS, z0 + CORNER), b: at(RADIUS, TOP) },
        ProfileEdge::Line { a: at(RADIUS, TOP), b: at(0., TOP) },
        ProfileEdge::Line { a: at(0., TOP), b: at(0., z0) },
    ];
    let meridian = Profile { origin: FOOT, normal: [0., 1., 0.], loops: vec![edges],
                             names: Vec::new() };
    revolve(&meridian, FOOT, [0., 0., 1.], TAU)
}

/// The mill carried without turning along the curve: at `t` moved by the curve's point there less
/// its first, at the curve's rate.
#[derive(Debug)]
struct Along(BSpline);

impl Path for Along {
    fn at(&self, t: f64) -> Result<Motion, KernelError> {
        let (point, rate) = self.0.d1(t);
        Motion::translation(sub(point, CONTROL[0]), rate)
            .map_err(|e| KernelError::from(format!("the curve's pose at {t}: {e:?}")))
    }

    /// What the motion does over any stretch, enclosed, as the kernel's membership reads its speed
    /// bound: the curve within its poles' box (a Bézier curve lies in its control polygon's hull),
    /// so the move within that box less the first pole; no turn; its speed within the longest of
    /// its hodograph's poles.
    fn stretch(&self, _: f64, _: f64) -> Option<Stretch> {
        let moved = CONTROL.map(|p| sub(p, CONTROL[0]));
        let one = |i: usize, j: usize| Interval::point(if i == j { 1. } else { 0. }).ok();
        let turn = [[one(0, 0)?, one(0, 1)?, one(0, 2)?], [one(1, 0)?, one(1, 1)?, one(1, 2)?],
                    [one(2, 0)?, one(2, 1)?, one(2, 2)?]];
        let speed = hodograph().iter().map(|&h| norm(h)).fold(0., f64::max);
        Some(Stretch { point: hull(&moved)?, turn, speed, spin: 0., origin: [0.; 3] })
    }
}

/// The box holding `points`, as intervals.
fn hull(points: &[V]) -> Option<[Interval; 3]> {
    let side = |k: usize| {
        let (lo, hi) = points.iter().fold((f64::INFINITY, f64::NEG_INFINITY),
                                          |(lo, hi), p| (lo.min(p[k]), hi.max(p[k])));
        Interval::new(lo, hi).ok()
    };
    Some([side(0)?, side(1)?, side(2)?])
}

/// The Bézier curve's hodograph's poles, `3 (P[i+1] − P[i])`: its rate is a weighted mean of them.
fn hodograph() -> [V; 3] {
    std::array::from_fn(|i| scale(sub(CONTROL[i + 1], CONTROL[i]), 3.))
}

pub fn build() -> Result<Brep, String> {
    let stock = block(STOCK_LO, STOCK_HI)?;
    let tool = mill()?;
    // the curve as cad-kernel holds a Bézier segment: a cubic B-spline on clamped knots
    let knots = vec![0., 0., 0., 0., 1., 1., 1., 1.];
    let curve = BSpline::new(3, knots, CONTROL.to_vec())?;
    let path = Trajectory::General(Arc::new(Along(curve)));
    // the kernel's closed swept solid cut from the block on one arrangement: the sheet each
    // instant's contact sweeps, fitted, and the mill where it starts and where it stops
    let size = stock.size().max(tool.size());
    cut_from(&stock, &tool, &path, [0., 1.], BAR, tolerance(size))
        .map_err(|why| format!("the mill's sweep not built: {why:?}"))
}
