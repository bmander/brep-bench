//! A lemon: the segment of the disc of radius 5 about (−3, 0, 0) in the xz plane with x ≥ 0, cut
//! off by the z axis, turned once about z.
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::geom::Frame;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const CENTRE: f64 = -3.;
const RADIUS: f64 = 5.;

pub fn build() -> Result<Brep, String> {
    // the arc from (0, 0, −4) out through (2, 0, 0) to (0, 0, 4), closed down its chord on the axis
    let tip = (RADIUS * RADIUS - CENTRE * CENTRE).sqrt();
    let reach = (-CENTRE / RADIUS).acos();
    let frame = Frame::new([CENTRE, 0., 0.], [0., -1., 0.], [1., 0., 0.]);
    let (north, south) = ([0., 0., tip], [0., 0., -tip]);
    let section = Profile {
        origin: south,
        normal: [0., -1., 0.],
        loops: vec![vec![
            ProfileEdge::Arc { frame, r: RADIUS, span: Some([-reach, reach]) },
            ProfileEdge::Line { a: north, b: south },
        ]],
        names: Vec::new(),
    };
    revolve(&section, [0., 0., 0.], [0., 0., 1.], TAU)
}
