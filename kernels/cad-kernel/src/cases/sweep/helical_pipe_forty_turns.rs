//! A pipe of radius 1 along the helix of radius 10 about z, rising 5 a turn, from (10, 0, 0) for
//! forty right-handed turns to (10, 0, 200), square to the path, its ends flat: a coil spring.
//!
//! `sweep/helical-pipe`'s way, the ball's canal with the half ball beyond each end cut off, is
//! refused at forty turns: the cutter at an end, a plane, crosses every coil's tube ("a shell of
//! the arrangement is kept on one side of a plane and dropped on another"). A screw carries the
//! helix onto itself, and its tangent with it, so the disc square to the path at the start, screwed
//! forty turns, stays square to it: the pipe with its flat ends, in one sweep.
use cad_kernel::build::{disc, screwed};
use cad_kernel::topo::Brep;
use cad_kernel::util::space::unit;
use std::f64::consts::TAU;

const HELIX_RADIUS: f64 = 10.;
const PITCH: f64 = 5.;
const TURNS: f64 = 40.;
const RADIUS: f64 = 1.;

pub fn build() -> Result<Brep, String> {
    // the path's tangent at (10, 0, 0), where the disc starts
    let tangent = unit([0., TAU * HELIX_RADIUS, PITCH]);
    let section = disc([HELIX_RADIUS, 0., 0.], tangent, RADIUS);
    screwed(&section, [0., 0., 0.], [0., 0., 1.], TURNS * TAU, TURNS * PITCH)
}
