//! A pipe of radius 1 along 3 turns of the helix of radius 10 and pitch 5 about z; flat ends.
use cad_kernel::build::{canal, cut_before};
use cad_kernel::topo::Brep;
use cad_kernel::util::space::{scale, unit};
use std::f64::consts::TAU;

const HELIX_RADIUS: f64 = 10.;
const PITCH: f64 = 5.;
const TURNS: f64 = 3.;
const RADIUS: f64 = 1.;

pub fn build() -> Result<Brep, String> {
    let (axis_at, axis) = ([0., 0., 0.], [0., 0., 1.]);
    let start = [HELIX_RADIUS, 0., 0.];
    let end = [HELIX_RADIUS, 0., TURNS * PITCH];
    // the ball swept along the helix: the disc square to the path screwed along it, with the half
    // ball the ball leaves beyond each end disc
    let rolled = canal(start, RADIUS, axis_at, axis, TURNS * TAU, TURNS * PITCH, "pipe")?;
    // the path's tangent, the same at both ends after whole turns: each end disc lies square to it
    let tangent = unit([0., TAU * HELIX_RADIUS, PITCH]);
    // the half balls cut off at those discs, the cutters only as large as a ball
    let reach = 2. * RADIUS;
    let rolled = cut_before(&rolled, start, tangent, reach, 0.)?;
    cut_before(&rolled, end, scale(tangent, -1.), reach, 0.)
}
