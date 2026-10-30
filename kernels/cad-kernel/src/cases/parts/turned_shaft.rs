//! A shaft on z: radius 20 to z = 60, radius 12 on to z = 160, the inside corner rounded to 4.
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::geom::Frame;
use cad_kernel::topo::Brep;
use std::f64::consts::{PI, TAU};

const THICK_RADIUS: f64 = 20.;
const SHOULDER: f64 = 60.;
const THIN_RADIUS: f64 = 12.;
const LENGTH: f64 = 160.;
const ROUND: f64 = 4.;

pub fn build() -> Result<Brep, String> {
    // the section in the xz plane, its normal −y so that its frame reads x as x and z as up
    let normal = [0., -1., 0.];
    let at = |x: f64, z: f64| [x, 0., z];
    let line = |a: [f64; 3], b: [f64; 3]| ProfileEdge::Line { a, b };
    // the round: a quarter of the circle about (16, 64), from the shoulder at radius 16 (straight
    // below its centre, 270°) back to the thin part's side at z = 64 (straight inward, 180°)
    let centre = at(THIN_RADIUS + ROUND, SHOULDER + ROUND);
    let round = ProfileEdge::Arc {
        frame: Frame::new(centre, normal, [1., 0., 0.]),
        r: ROUND,
        span: Some([PI, 1.5 * PI]),
    };
    let section = Profile {
        origin: at(0., 0.),
        normal,
        loops: vec![vec![
            line(at(0., 0.), at(THICK_RADIUS, 0.)),
            line(at(THICK_RADIUS, 0.), at(THICK_RADIUS, SHOULDER)),
            line(at(THICK_RADIUS, SHOULDER), at(THIN_RADIUS + ROUND, SHOULDER)),
            round,
            line(at(THIN_RADIUS, SHOULDER + ROUND), at(THIN_RADIUS, LENGTH)),
            line(at(THIN_RADIUS, LENGTH), at(0., LENGTH)),
            line(at(0., LENGTH), at(0., 0.)),
        ]],
        names: Vec::new(),
    };
    revolve(&section, at(0., 0.), [0., 0., 1.], TAU)
}
