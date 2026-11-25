//! The cone on the z axis from its base of radius 6 at z = 0 to its apex at z = 12, its tip rounded
//! by a ball of radius 1 tangent to its side, shrunk by 1.5: the ball's face, moved in by more than
//! its radius, drops, and the moved side runs on to its own apex.
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::geom::Frame;
use cad_kernel::offset::{shrunk, Join};
use cad_kernel::topo::Brep;
use std::f64::consts::{FRAC_PI_2, TAU};

const BASE_RADIUS: f64 = 6.;
const APEX: f64 = 12.;
const TIP: f64 = 1.;
const SHRINK: f64 = 1.5;

pub fn build() -> Result<Brep, String> {
    // the side leans in 1 for every 2 it rises: in the half-section its outward normal is
    // (2, 1)/√5, and the ball of radius TIP touching it has its centre on the axis TIP·√5 below
    // the apex, and touches it TIP from there along that normal
    let s5 = 5f64.sqrt();
    let centre = APEX - TIP * s5;
    let touch = [2. * TIP / s5, 0., centre + TIP / s5];
    // the half-section in the plane y = 0: out along the base, up the side to where the ball
    // touches it, round the ball (from the normal's angle, atan ½, to the axis), down the axis
    let ball = Frame::new([0., 0., centre], [0., -1., 0.], [1., 0., 0.]);
    let edges = vec![
        ProfileEdge::Line { a: [0., 0., 0.], b: [BASE_RADIUS, 0., 0.] },
        ProfileEdge::Line { a: [BASE_RADIUS, 0., 0.], b: touch },
        ProfileEdge::Arc { frame: ball, r: TIP, span: Some([0.5f64.atan(), FRAC_PI_2]) },
        ProfileEdge::Line { a: [0., 0., centre + TIP], b: [0., 0., 0.] },
    ];
    let section = Profile { origin: [0., 0., 0.], normal: [0., -1., 0.], loops: vec![edges],
                            names: Vec::new() };
    let tipped = revolve(&section, [0., 0., 0.], [0., 0., 1.], TAU)?;
    shrunk(&tipped, SHRINK, Join::Round)
}
