//! A rod, the cylinder of radius 5 about z from z = 0 to 20, with a helical groove ground round it
//! by an oblate spheroid — centred at (10.5, 0, 0), semi-axes 6, 6 and 1.5, its short axis along
//! (0, cos h, −sin h), h = atan(2π·4.75 / 30) — screwed about z, right-handed and rising 30 a turn,
//! from a turn of −90° to +330°.
//!
//! The kernel's closed sweep (`swept::closed::cut_from`) takes the wheel whole: under a screw its
//! closed class takes planes, balls, and cylinders and cones about axes along the screw's, and
//! refuses the spheroid (`Unbuilt::Curved`), whose face is a ball's affine image, its contact
//! curves no closed form the candidates hold (an affine map does not carry a screw to a screw, so
//! the slid ellipsoid's route through the ball does not apply); since 0.9.0 such a tool goes the
//! contact's road — the spheroid's contact with the screw carried over the roll as sheets, given
//! their meetings with one another and with the end poses, trimmed by membership in the sweep, and
//! the result cut from the rod.
use cad_kernel::build::{disc, ellipsoid, prism};
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::TAU;

const ROD_RADIUS: f64 = 5.;
const ROD_LENGTH: f64 = 20.;
const CENTRE: V = [10.5, 0., 0.];
const SEMI: [f64; 3] = [6., 6., 1.5];
const LEAD: f64 = 30.;
const ROLL: [f64; 2] = [-90f64.to_radians(), 330f64.to_radians()];
/// How closely the wheel is meshed for the sweep's membership reads: a hundred-and-fiftieth of its
/// short semi-axis.
const BAR: f64 = 0.01;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let rod = prism(&disc([0., 0., 0.], up, ROD_RADIUS), 0., ROD_LENGTH)?;
    // the wheel's axis square to x, leaning from y down toward −z by the helix's angle at 4.75
    let h = (TAU * 4.75 / LEAD).atan();
    let axis = [0., h.cos(), -h.sin()];
    // the spheroid on a frame about that axis: its two long semi-axes square to it (the first
    // along x), its short one along it
    let wheel = ellipsoid(CENTRE, axis, [1., 0., 0.], SEMI)?;
    // the screw about z: a radian a radian of the roll, rising the lead a turn
    let path = Trajectory::Twist(Screw {
        origin: [0., 0., 0.], axis: up, ratio: 1., advance: LEAD,
    });
    let size = rod.size().max(wheel.size());
    cut_from(&rod, &wheel, &path, ROLL, BAR, tolerance(size))
        .map_err(|why| format!("the wheel's sweep not built: {why:?}"))
}
