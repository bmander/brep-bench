//! A rod, the cylinder of radius 5 about z from z = 0 to 20, grooved all round by a wheel with a
//! sharp V rim — the triangle (4, 0, 10), (6.5, 0, 10) ± 2.5 tan 30°·(0, sin 20°, cos 20°) turned
//! about the line through (14, 0, 10) along (0, sin 20°, cos 20°) — turned a whole turn about z.
//!
//! Sweep the wheel a whole turn into a closed solid, then subtract that solid from the rod. The
//! kernel builds a whole turn by the wheel's shadow in a half-plane through the axis (#880 Q1,
//! `swept::shadow`): its flanks' and bore's contacts and its three sharp circles laid in that
//! half-plane, the shadow's boundary kept by membership and turned once.
use crate::sketch::polygon;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism, revolve};
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::closed;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use cad_kernel::util::space::{add, scale};
use std::f64::consts::TAU;

const ROD_RADIUS: f64 = 5.;
const ROD_LENGTH: f64 = 20.;
const CENTRE: V = [14., 0., 10.];
const LEAN: f64 = 20f64.to_radians();
/// How closely the wheel is meshed for the sweep's membership reads.
const BAR: f64 = 0.01;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let rod = prism(&disc([0., 0., 0.], up, ROD_RADIUS), 0., ROD_LENGTH)?;
    let axis = [0., LEAN.sin(), LEAN.cos()];
    // the triangle in the plane through the axis and the rim's lowest point, turned once about
    // the axis: two cones meeting in the sharp rim, and the bore's cylinder
    let half = scale(axis, 2.5 * 30f64.to_radians().tan());
    let back = [6.5, 0., 10.];
    let triangle = polygon(&[[4., 0., 10.], add(back, half), add(back, scale(half, -1.))]);
    let wheel = revolve(&triangle, CENTRE, axis, TAU)?;
    // the turn about z: a screw that does not advance, a whole turn over a roll of τ
    let path = Trajectory::Twist(Screw { origin: [0., 0., 0.], axis: up, ratio: 1., advance: 0. });
    // the kernel's swept solid, a whole turn: its shadow's solid of revolution
    let tol = tolerance(rod.size().max(wheel.size()));
    let swept = closed(&wheel, &path, [0., TAU], BAR, tol)
        .map_err(|why| format!("the wheel's sweep not built: {why:?}"))?;
    Ok(combined(&rod, &swept, Op::Cut, tol)?)
}
