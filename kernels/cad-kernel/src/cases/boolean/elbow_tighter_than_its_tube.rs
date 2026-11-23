//! The outside of a pipe elbow whose bend is tighter than the pipe is thick: a round tube of
//! radius 20 turned a quarter turn about the y axis, only 18 from its centre-line, with a straight
//! leg of length 40 at each end. The turn is everything the disk of radius 20 about (0, 0, 18) in
//! the plane x = 0 passes through as it turns 90° about the y axis, right-handed (+z to +x); the
//! legs are the cylinders of radius 20 about the line y = 0, z = 18 from x = −40 to 0 and about
//! the line x = 18, y = 0 from z = −40 to 0. The solid is the union of the turn and the legs; the
//! turn's outer surface is a piece of a spindle torus whose apexes lie on the bend axis.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism, revolve, Profile, ProfileEdge};
use cad_kernel::geom::{Frame, V};
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_PI_2;

const TUBE: f64 = 20.;
const BEND: f64 = 18.;
const LEG: f64 = 40.;
const ALONG: V = [0., 1., 0.];

pub fn build() -> Result<Brep, String> {
    // the disk reaches 2 past the bend axis, which a section turned about it may not cross
    // (`build::revolve` refuses "a revolution's profile crosses its axis"): it is clipped by its
    // chord on the axis, from (0, −h, 0) to (0, h, 0), as the kernel's own elbow (#996) is. What
    // the clipped piece sweeps, at most 2 from the axis on the far side, turned between −z and
    // −x, lies in both legs (a point d past the axis, turned by θ, stands nearer the leg's axis
    // than d past it did), so the union is the same
    let h = (TUBE * TUBE - BEND * BEND).sqrt();
    // the arc measured from +z, turning toward −y (the frame's y, x × z), past the top round to
    // where it meets the axis either side
    let reach = (-BEND / TUBE).acos();
    let frame = Frame::new([0., 0., BEND], [1., 0., 0.], [0., 0., 1.]);
    let rim = ProfileEdge::Arc { frame, r: TUBE, span: Some([-reach, reach]) };
    let chord = ProfileEdge::Line { a: [0., -h, 0.], b: [0., h, 0.] };
    let loops = vec![vec![rim, chord]];
    let section = Profile { origin: [0., 0., BEND], normal: [1., 0., 0.], loops, names: vec![] };
    let turn = revolve(&section, [0., 0., 0.], ALONG, FRAC_PI_2)?;
    // the legs, each from its far end up to the turn's end it stands on
    let first = prism(&disc([-LEG, 0., BEND], [1., 0., 0.], TUBE), 0., LEG)?;
    let second = prism(&disc([BEND, 0., -LEG], [0., 0., 1.], TUBE), 0., LEG)?;
    let legs = combined(&first, &second, Op::Union, 0.)?;
    Ok(combined(&turn, &legs, Op::Union, 0.)?)
}
