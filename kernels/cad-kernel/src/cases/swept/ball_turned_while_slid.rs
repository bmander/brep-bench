//! A ball of radius 1 centred at (5, 0, 0), at the end of an arm of length 5 from the z axis,
//! swept by a rigid motion that turns it about the z axis while sliding it along x, both at a
//! steady rate: at each instant t from 0 to 1 the ball is turned right-handed about z by 180°·t and
//! then slid by (25t, 0, 0). The solid is everything the ball passes through: its centre runs along
//! a curtate trochoid from (5, 0, 0) to (20, 0, 0), no tighter than the ball's radius.
//!
//! The kernel's sweep of a ball (`swept::canal`): it reads the ball's centre's path off the
//! trajectory and carries the ball's disc square to it, a face family no closed form carries,
//! capped by a half ball at each end (the recipe's `Canal` node, as a document's ball sweep is
//! built).
use cad_kernel::build::sphere;
use cad_kernel::geom::V;
use cad_kernel::recipe::{node, Node};
use cad_kernel::swept::canal::{ball, tube};
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};

const UP: V = [0., 0., 1.];
const CENTRE: V = [5., 0., 0.];
const RADIUS: f64 = 1.;
/// How far the ball slides along x over the roll; it turns half a turn meanwhile.
const SLIDE: f64 = 25.;

pub fn build() -> Result<Brep, String> {
    let tool = sphere(CENTRE, UP, RADIUS)?;
    // the turn about z, π radians a unit of the roll
    let turn = Trajectory::Twist(Screw { origin: [0.; 3], axis: UP, ratio: PI, advance: 0. });
    // then the slide: seen from an observer sliding back along x, `SLIDE` a unit of the roll (a
    // slide's advance is read per turn of the parameter, 2π of it)
    let slide = Screw { origin: [0.; 3], axis: [-1., 0., 0.], ratio: 0., advance: SLIDE * TAU };
    let back = Trajectory::Twist(slide);
    let path = Trajectory::Relative { source: Box::new(turn), observer: Box::new(back) };
    // the ball read from its B-rep, as the kernel's sweep reads its tool
    let (centre, radius, face) = ball(&tool, 1.).ok_or("the ball is not read as a ball")?;
    let swept = tube(&path, centre, radius, &face, [0., 1.])
        .map_err(|why| format!("the ball's tube not built: {why:?}"))?
        .ok_or("the ball's tube is a screw's meeting itself")?;
    let canal =
        Node::Canal { centre: swept.centre, radius: swept.radius, spine: swept.spine, face };
    node(&canal, &BTreeMap::new())
}
