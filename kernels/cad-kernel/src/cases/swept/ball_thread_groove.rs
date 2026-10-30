//! A rod, the cylinder of radius 5 about z from z = 0 to 20, with a helical groove cut round it by
//! a ball of radius 1 whose centre runs six right-handed turns of the helix of radius 5 rising 4 a
//! turn, from (5, 0, −2) to (5, 0, 22).
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{canal, disc, prism};
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const ROD_RADIUS: f64 = 5.;
const ROD_LENGTH: f64 = 20.;
const RADIUS: f64 = 1.;
const HELIX_RADIUS: f64 = 5.;
const RISE: f64 = 4.;
const TURNS: f64 = 6.;
const START_Z: f64 = -2.;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let rod = prism(&disc([0., 0., 0.], up, ROD_RADIUS), 0., ROD_LENGTH)?;
    // everything the ball passes through as it is screwed along z, by cad-kernel's own sweep of a
    // ball: the canal of six turns and a rise of 4 each, its disc square to the helix screwed
    // along it, with the half ball beyond each end disc (both ends clear of the rod)
    let start = [HELIX_RADIUS, 0., START_Z];
    let swept = canal(start, RADIUS, [0., 0., 0.], up, TURNS * TAU, TURNS * RISE, "groove")?;
    Ok(combined(&rod, &swept, Op::Cut, 0.)?)
}
