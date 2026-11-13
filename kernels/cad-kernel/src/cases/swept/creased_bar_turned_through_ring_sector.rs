//! A ring sector (radii 8 to 12 about z, z from −1 to 1, polar angle 0° to 60°) grooved by a bar
//! turned about z, right-handed, from −140° to +140°: the rectangle 10 ≤ a ≤ 13, |b| ≤ 2 at
//! a u + b v, u = (1, 0, 0), v = (0, cos 20°, −sin 20°), carried 4 each way along its normal
//! n = (0, sin 20°, cos 20°), its four sides drafted 2° inward both ways from that plane.
//!
//! Built by the kernel's closed sweep cut from the sector (`swept::closed::cut_from`), whose
//! arrangement meets the bar's edges with its turned candidates, surfaces of revolution.
use crate::sketch::polygon;
use cad_kernel::build::{drafted, revolve};
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use cad_kernel::util::space::{add, scale};
use std::f64::consts::FRAC_PI_3;

const UP: V = [0., 0., 1.];
/// How far the section's plane leans across the turn, in degrees.
const LEAN: f64 = 20.;
const A: [f64; 2] = [10., 13.];
const HALF_B: f64 = 2.;
/// How far the bar runs either way from its section's plane, and its sides' draft, in degrees.
const REACH: f64 = 4.;
const DRAFT: f64 = 2.;
/// How far the turn carries the bar each way from where it is drawn, in degrees.
const TURN: f64 = 140.;

/// The ring sector: its section in the half-plane at 0° turned 60° about z.
fn sector() -> Result<Brep, String> {
    let section = polygon(&[[8., 0., -1.], [12., 0., -1.], [12., 0., 1.], [8., 0., 1.]]);
    revolve(&section, [0.; 3], UP, FRAC_PI_3)
}

pub fn build() -> Result<Brep, String> {
    let blank = sector()?;
    let (s, c) = LEAN.to_radians().sin_cos();
    let (u, v) = ([1., 0., 0.], [0., c, -s]);
    // the section, walked counter-clockwise in (a, b), so that its normal is u × v = n
    let at = |a: f64, b: f64| add(scale(u, a), scale(v, b));
    let section = polygon(&[at(A[0], -HALF_B), at(A[1], -HALF_B), at(A[1], HALF_B),
                            at(A[0], HALF_B)]);
    // the kernel's drafted prism: each side tapered into the material away from the section's
    // plane, either way, the two halves glued at the parting plane in a convex crease
    let bar = drafted(&section, -REACH, REACH, DRAFT.to_radians())?;
    // the turn about z: a screw that does not advance, a radian a radian of the roll
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: UP, ratio: 1., advance: 0. });
    // the kernel's closed swept solid cut from the sector on one arrangement (a polyhedron's flat
    // faces mesh exactly, at any bar)
    let size = blank.size().max(bar.size());
    let turn = TURN.to_radians();
    cut_from(&blank, &bar, &path, [-turn, turn], size, tolerance(size))
        .map_err(|why| format!("the bar's sweep not built: {why:?}"))
}
