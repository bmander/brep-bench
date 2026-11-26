//! The solid of `swept/ball-screwed-round-a-tight-helix` turned rigidly so that its axis is
//! w = (1, 2, 2)/3: a ball of radius 2 whose centre runs from 1.5u, u = (2, −2, 1)/3, along the helix
//! of radius 1.5 about w, rising 3 a turn, for two turns.
use cad_kernel::build::sphere;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::closed;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use cad_kernel::util::space::scale;
use std::f64::consts::TAU;

/// The turned frame: u where the case before it had x, w where it had z.
const U: V = [2. / 3., -2. / 3., 1. / 3.];
const W: V = [1. / 3., 2. / 3., 2. / 3.];
const RADIUS: f64 = 2.;
/// How far the helix stands from its axis, how far the screw rises a whole turn, and how many
/// turns it makes.
const HELIX_RADIUS: f64 = 1.5;
const RISE: f64 = 3.;
const TURNS: f64 = 2.;
/// How closely the ball is meshed for the sweep's membership reads: a thousandth of its radius.
const BAR: f64 = 0.002;

pub fn build() -> Result<Brep, String> {
    let ball = sphere(scale(U, HELIX_RADIUS), W, RADIUS)?;
    // the screw about w: a radian a radian of the roll, rising `RISE` a turn
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: W, ratio: 1., advance: RISE });
    let size = ball.size() + RISE * TURNS;
    closed(&ball, &path, [0., TURNS * TAU], BAR, tolerance(size))
        .map_err(|why| format!("the ball's sweep not built: {why:?}"))
}
