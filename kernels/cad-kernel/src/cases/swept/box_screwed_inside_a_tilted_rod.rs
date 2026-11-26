//! The rod and tunnel of `swept/box-screwed-inside-a-rod` turned rigidly so that the rod's axis is
//! w = (1, 2, 2)/3: the cylinder of radius 5 about w from the origin to 20w, less everything the unit
//! cube 2.5 ≤ p·u ≤ 3.5, −0.5 ≤ p·v ≤ 0.5, 2 ≤ p·w ≤ 3 passes through as it is screwed about w,
//! rising 4 a turn, for two turns: a sealed helical cavity.
use crate::sketch::block;
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::{tolerance, Frame, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::TAU;

/// The turned frame: u where the case before it had x, w where it had z (v = w × u).
const U: V = [2. / 3., -2. / 3., 1. / 3.];
const W: V = [1. / 3., 2. / 3., 2. / 3.];
const ROD_RADIUS: f64 = 5.;
const ROD_LENGTH: f64 = 20.;
/// The box in (u, v, w).
const BOX_LO: V = [2.5, -0.5, 2.];
const BOX_HI: V = [3.5, 0.5, 3.];
/// How far the screw rises a whole turn, and how many turns it makes.
const RISE: f64 = 4.;
const TURNS: f64 = 2.;

pub fn build() -> Result<Brep, String> {
    let rod = prism(&disc([0.; 3], W, ROD_RADIUS), 0., ROD_LENGTH)?;
    // the box drawn in (u, v, w) and placed: the point (x, y, z) at xu + yv + zw
    let tool = block(BOX_LO, BOX_HI)?.moved(&Frame::new([0.; 3], W, U).placing());
    // the screw about w: a radian a radian of the roll, rising `RISE` a turn
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: W, ratio: 1., advance: RISE });
    let size = rod.size();
    cut_from(&rod, &tool, &path, [0., TURNS * TAU], size, tolerance(size))
        .map_err(|why| format!("the box's sweep not built: {why:?}"))
}
