//! The rod and tunnel of `swept/box-screwed-inside-a-rod`, a hundred times larger: the cylinder of
//! radius 500 about z from z = 0 to 2000, less everything the box 250 ≤ x ≤ 350, −50 ≤ y ≤ 50,
//! 200 ≤ z ≤ 300 passes through as it is screwed about z, rising 400 a turn, for two turns: a
//! sealed helical cavity.
use crate::sketch::block;
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::TAU;

const ROD_RADIUS: f64 = 500.;
const ROD_LENGTH: f64 = 2000.;
const BOX_LO: V = [250., -50., 200.];
const BOX_HI: V = [350., 50., 300.];
/// How far the screw rises a whole turn, and how many turns it makes.
const RISE: f64 = 400.;
const TURNS: f64 = 2.;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let rod = prism(&disc([0.; 3], up, ROD_RADIUS), 0., ROD_LENGTH)?;
    let tool = block(BOX_LO, BOX_HI)?;
    // the screw about z: a radian a radian of the roll, rising `RISE` a turn
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: up, ratio: 1., advance: RISE });
    // the kernel's closed swept solid cut from the rod on one arrangement (a box's flat faces mesh
    // exactly, at any bar)
    let size = rod.size();
    cut_from(&rod, &tool, &path, [0., TURNS * TAU], size, tolerance(size))
        .map_err(|why| format!("the box's sweep not built: {why:?}"))
}
