//! The solid a ball of radius 2 about (1, 0, 0) fills as it turns rigidly about the z axis,
//! right-handed, by 120°: the axis passes through the ball, so the tube about its centre's arc is a
//! piece of a spindle torus.
//!
//! The kernel's closed sweep (`swept::closed::closed`) takes the ball whole: its closed class
//! refuses a ball reaching across the axis (`Unbuilt::Curved`), and since 0.10.0 such a tool goes
//! the contact's road under a turn — the ball's contact, its great circle in the plane through the
//! axis and its centre, cut where it crosses the axis, the arc beyond it buried and the rest turned
//! into one spindle torus closing at the axis at both ends, between the ball's two end poses.
use cad_kernel::build::sphere;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::closed;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};

const UP: V = [0., 0., 1.];
const CENTRE: V = [1., 0., 0.];
const RADIUS: f64 = 2.;
/// The turn, in degrees.
const TURN: f64 = 120.;
/// How closely the ball is meshed for the sweep's membership reads: a thousandth of its radius.
const BAR: f64 = 0.002;

pub fn build() -> Result<Brep, String> {
    let ball = sphere(CENTRE, UP, RADIUS)?;
    // the turn about z: a screw that does not advance, a radian a radian of the roll
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: UP, ratio: 1., advance: 0. });
    // the kernel's closed swept solid: a ball's great circle in the plane through the axis and its
    // centre turned into a torus — here one reaching across the axis
    closed(&ball, &path, [0., TURN.to_radians()], BAR, tolerance(ball.size()))
        .map_err(|why| format!("the ball's sweep not built: {why:?}"))
}
