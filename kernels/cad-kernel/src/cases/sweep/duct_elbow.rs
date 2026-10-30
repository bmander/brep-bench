//! A square 18 × 18 with a hole 14 × 14, about (30, 0, 0) in the xz plane, turned 90° about z.
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::geom::V;
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_PI_2;

const BEND_RADIUS: f64 = 30.;
const HALF_WIDTH: f64 = 9.;
const WALL: f64 = 2.;

/// The square in the xz plane of half-width `half` about the arc's start, as four lines.
fn square(half: f64) -> Vec<ProfileEdge> {
    let at = |x: f64, z: f64| [BEND_RADIUS + x, 0., z];
    let corners: [V; 4] = [at(-half, -half), at(half, -half), at(half, half), at(-half, half)];
    (0..4).map(|k| ProfileEdge::Line { a: corners[k], b: corners[(k + 1) % 4] }).collect()
}

pub fn build() -> Result<Brep, String> {
    // the section: the outer square and its hole, in the vertical plane through the z axis
    let section = Profile {
        origin: [BEND_RADIUS, 0., 0.],
        normal: [0., -1., 0.],
        loops: vec![square(HALF_WIDTH), square(HALF_WIDTH - WALL)],
        names: Vec::new(),
    };
    // a sweep round an arc square to its plane's axis is a revolution: from +x towards +y
    revolve(&section, [0., 0., 0.], [0., 0., 1.], FRAC_PI_2)
}
