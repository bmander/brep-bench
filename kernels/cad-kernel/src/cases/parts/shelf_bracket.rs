//! An L bracket in the xz plane, a post 20 × 120 and an arm out to x = 100, corner rounded 8; y ±5.
use cad_kernel::build::{prism, Profile, ProfileEdge};
use cad_kernel::geom::Frame;
use cad_kernel::topo::Brep;
use std::f64::consts::{FRAC_PI_2, PI};

const POST_WIDTH: f64 = 20.;
const HEIGHT: f64 = 120.;
const ARM_DEPTH: f64 = 20.;
const REACH: f64 = 100.;
const ROUND: f64 = 8.;
const HALF_THICKNESS: f64 = 5.;

pub fn build() -> Result<Brep, String> {
    // the outline in the xz plane, its normal −y so that its frame reads x as x and z as up
    let normal = [0., -1., 0.];
    let at = |x: f64, z: f64| [x, 0., z];
    let line = |a: [f64; 3], b: [f64; 3]| ProfileEdge::Line { a, b };
    let under_arm = HEIGHT - ARM_DEPTH;
    // the round: a quarter of the circle about (28, 92), from the arm's underside at x = 28
    // (straight above its centre, 90°) to the post at z = 92 (straight back, 180°)
    let centre = at(POST_WIDTH + ROUND, under_arm - ROUND);
    let round = ProfileEdge::Arc {
        frame: Frame::new(centre, normal, [1., 0., 0.]),
        r: ROUND,
        span: Some([FRAC_PI_2, PI]),
    };
    let outline = Profile {
        origin: at(0., 0.),
        normal,
        loops: vec![vec![
            line(at(0., 0.), at(POST_WIDTH, 0.)),
            line(at(POST_WIDTH, 0.), at(POST_WIDTH, under_arm - ROUND)),
            round,
            line(at(POST_WIDTH + ROUND, under_arm), at(REACH, under_arm)),
            line(at(REACH, under_arm), at(REACH, HEIGHT)),
            line(at(REACH, HEIGHT), at(0., HEIGHT)),
            line(at(0., HEIGHT), at(0., 0.)),
        ]],
        names: Vec::new(),
    };
    // swept along its normal from −5 to 5: y from 5 to −5
    prism(&outline, -HALF_THICKNESS, HALF_THICKNESS)
}
