//! A spindle torus about z, its tube's middle circle radius 3 and its tube 5: the disc
//! (x − 3)² + z² ≤ 25 turned once about z, counted once, which is the turn of its part with x ≥ 0.
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::geom::Frame;
use cad_kernel::topo::Brep;
use std::f64::consts::{PI, TAU};

const MAJOR: f64 = 3.;
const MINOR: f64 = 5.;

pub fn build() -> Result<Brep, String> {
    // the outer arc, from (0, 0, −4) round through (8, 0, 0) to (0, 0, 4), closed down the axis;
    // its frame turns counter-clockwise about −y from +x towards +z
    let pole = (MINOR * MINOR - MAJOR * MAJOR).sqrt();
    let reach = PI - (MAJOR / MINOR).acos();
    let frame = Frame::new([MAJOR, 0., 0.], [0., -1., 0.], [1., 0., 0.]);
    let (north, south) = ([0., 0., pole], [0., 0., -pole]);
    let section = Profile {
        origin: south,
        normal: [0., -1., 0.],
        loops: vec![vec![
            ProfileEdge::Arc { frame, r: MINOR, span: Some([-reach, reach]) },
            ProfileEdge::Line { a: north, b: south },
        ]],
        names: Vec::new(),
    };
    revolve(&section, [0., 0., 0.], [0., 0., 1.], TAU)
}
