//! A horn torus about z centred on the origin: the disc (x − 5)² + z² ≤ 25 of the xz plane, which
//! touches the axis at the origin, turned once about z.
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::geom::Frame;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const MAJOR: f64 = 5.;
const MINOR: f64 = 5.;

pub fn build() -> Result<Brep, String> {
    // `torus` takes only a ring torus, its tube clear of the axis: the tube's circle turned instead
    let centre = [MAJOR, 0., 0.];
    let frame = Frame::new(centre, [0., -1., 0.], [1., 0., 0.]);
    let circle = ProfileEdge::Arc { frame, r: MINOR, span: None };
    let section = Profile { origin: centre, normal: [0., -1., 0.], loops: vec![vec![circle]],
                            names: Vec::new() };
    revolve(&section, [0., 0., 0.], [0., 0., 1.], TAU)
}
