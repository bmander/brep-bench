//! A rod, the cylinder of radius 5 about z from z = 0 to 20, with a helical groove ground round it
//! by a torus of radii 6 and 2, centred at (11.5, 0, 0) with its axis (0, cos h, −sin h), h =
//! atan(2π·5.5 / 30), screwed about z — rising 30 a turn — from a turn of −108° to +348°.
//!
//! The kernel's closed sweep (`swept::closed::cut_from`) takes the torus whole: its closed class
//! refuses a torus under a screw (`Unbuilt::Curved`), and since 0.9.0 such a tool goes the
//! contact's road — the torus's contact with the screw carried over the roll as sheets, given their
//! meetings with one another and with the end poses, trimmed by membership in the sweep, and the
//! result cut from the rod. No envelope is computed here.
use cad_kernel::build::{disc, prism, torus};
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::TAU;

const UP: V = [0., 0., 1.];
/// How far the screw rises a whole turn.
const RISE: f64 = 30.;
/// How far the screw turns from where the wheel is drawn, in degrees: down to, and up to.
const TURNS: [f64; 2] = [-108., 348.];
/// How closely the wheel is meshed for the sweep's membership reads.
const BAR: f64 = 0.01;

pub fn build() -> Result<Brep, String> {
    let rod = prism(&disc([0.; 3], UP, 5.), 0., 20.)?;
    // the wheel set along the helix the screw carries (5.5, 0, 0) along
    let h = (TAU * 5.5 / RISE).atan();
    let wheel = torus([11.5, 0., 0.], [0., h.cos(), -h.sin()], 6., 2.)?;
    // the screw about z: a radian a radian of the roll, rising `RISE` a turn
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: UP, ratio: 1., advance: RISE });
    let size = rod.size().max(wheel.size());
    cut_from(&rod, &wheel, &path, TURNS.map(f64::to_radians), BAR, tolerance(size))
        .map_err(|why| format!("the wheel's sweep not built: {why:?}"))
}
