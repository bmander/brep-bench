//! A rod, the cylinder of radius 5 about z from z = 0 to 20, with a helical V groove ground round
//! it by a wheel with a sharp 60° V rim — the triangle (4, 0, 0), (6.5, 0, 0) ± 2.5·tan 30°·along
//! turned about the axis through (14, 0, 0) along (0, cos h, −sin h), h = atan(2π·4.5 / 30) —
//! screwed about z, rising 30 a turn, from a turn of −144° to +384°.
//!
//! The kernel's closed sweep (`swept::closed::cut_from`) takes the wheel whole: its closed class
//! refuses cones about an axis leaning to the screw (`Unbuilt::Curved`), and since 0.9.0 such a
//! tool goes the contact's road — each flank's contact carried over the roll as a sheet, the sharp
//! rim's fan (where the two flanks face either way along the motion) as a sheet of its own, given
//! their meetings and the end poses, trimmed by membership in the sweep, and the result cut from
//! the rod. No envelope is computed here.
use crate::sketch::polygon;
use cad_kernel::build::{disc, prism, revolve};
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use cad_kernel::util::space::{add, scale, sub};
use std::f64::consts::TAU;

const UP: V = [0., 0., 1.];
/// The wheel's axis passes through here.
const HUB: V = [14., 0., 0.];
/// The rim's sharp edge, nearest the rod.
const TIP: V = [4., 0., 0.];
/// How far the screw rises a whole turn.
const RISE: f64 = 30.;
/// How far the screw turns from where the wheel is drawn, in degrees: down to, and up to.
const TURNS: [f64; 2] = [-144., 384.];
/// How closely the wheel is meshed for the sweep's membership reads.
const BAR: f64 = 0.01;

pub fn build() -> Result<Brep, String> {
    let rod = prism(&disc([0.; 3], UP, 5.), 0., 20.)?;
    // the wheel set along the helix the screw carries (4.5, 0, 0) along
    let h = (TAU * 4.5 / RISE).atan();
    let along = [0., h.cos(), -h.sin()];
    let half = scale(along, 2.5 * 30f64.to_radians().tan());
    let back = add(TIP, [2.5, 0., 0.]);
    let wheel = revolve(&polygon(&[TIP, add(back, half), sub(back, half)]), HUB, along, TAU)?;
    // the screw about z: a radian a radian of the roll, rising `RISE` a turn
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: UP, ratio: 1., advance: RISE });
    let size = rod.size().max(wheel.size());
    cut_from(&rod, &wheel, &path, TURNS.map(f64::to_radians), BAR, tolerance(size))
        .map_err(|why| format!("the wheel's sweep not built: {why:?}"))
}
