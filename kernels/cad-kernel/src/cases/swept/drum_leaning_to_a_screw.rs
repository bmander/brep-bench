//! A ring sector (radii 8 to 12 about z, z from −1 to 1, polar angle 0° to 60°) with a helical
//! groove milled through it by a drum — the cylinder of radius 0.8 and length 3 whose middle is
//! (10, 0, 0) and whose axis runs along (0, sin 25°, cos 25°) — screwed about z, right-handed and
//! rising 1 a turn, from a turn of −140° to +140°.
//!
//! The kernel's closed sweep (`swept::closed::cut_from`) takes the drum whole: its closed class
//! refuses a cylinder about an axis leaning to the screw (`Unbuilt::Curved`), and since 0.9.0 such
//! a tool goes the contact's road — the side's contact and each sharp rim's fan carried over the
//! roll as sheets, given their meetings and the end poses, trimmed by membership in the sweep, and
//! the result cut from the sector.
use crate::sketch::polygon;
use cad_kernel::build::{disc, prism, revolve};
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use cad_kernel::util::space::{scale, sub};
use std::f64::consts::FRAC_PI_3;

const UP: V = [0., 0., 1.];
const MIDDLE: V = [10., 0., 0.];
const RADIUS: f64 = 0.8;
const LENGTH: f64 = 3.;
/// How far the drum's axis leans from upright toward +y, in degrees.
const LEAN: f64 = 25.;
/// How far the screw rises a whole turn.
const RISE: f64 = 1.;
/// How far the screw turns each way from where the drum is drawn, in degrees.
const TURN: f64 = 140.;
/// How closely the drum is meshed for the sweep's membership reads.
const BAR: f64 = 0.002;

/// The ring sector: its section in the half-plane at 0° turned 60° about z.
fn sector() -> Result<Brep, String> {
    let section = polygon(&[[8., 0., -1.], [12., 0., -1.], [12., 0., 1.], [8., 0., 1.]]);
    revolve(&section, [0.; 3], UP, FRAC_PI_3)
}

pub fn build() -> Result<Brep, String> {
    let blank = sector()?;
    let lean = LEAN.to_radians();
    let along = [0., lean.sin(), lean.cos()];
    let drum = prism(&disc(sub(MIDDLE, scale(along, LENGTH / 2.)), along, RADIUS), 0., LENGTH)?;
    // the screw about z: a radian a radian of the roll, rising `RISE` a turn
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: UP, ratio: 1., advance: RISE });
    let size = blank.size().max(drum.size());
    let turn = TURN.to_radians();
    cut_from(&blank, &drum, &path, [-turn, turn], BAR, tolerance(size))
        .map_err(|why| format!("the drum's sweep not built: {why:?}"))
}
