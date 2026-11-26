//! The cylinder of `fillet/elliptic-section-cylinder-rim` at a hundredth of its size: radius 0.05
//! on the z axis from z = 0 to 0.1, its top rim rounded 0.01 across the top and 0.03 down the side
//! by a quarter ellipse turned about the axis.
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::nurbs::BSpline;
use cad_kernel::topo::Brep;
use std::f64::consts::{FRAC_1_SQRT_2, TAU};
use std::sync::Arc;

const RADIUS: f64 = 0.05;
const HEIGHT: f64 = 0.1;
/// The round's legs: across the top and down the side.
const ACROSS: f64 = 0.01;
const DOWN: f64 = 0.03;

pub fn build() -> Result<Brep, String> {
    // the section in the half plane y = 0, x ≥ 0; the round's quarter ellipse a rational quadratic
    // on the corner of its box, weighted √2/2
    let at = |rho: f64, z: f64| [rho, 0., z];
    let poles = vec![at(RADIUS, HEIGHT - DOWN), at(RADIUS, HEIGHT), at(RADIUS - ACROSS, HEIGHT)];
    let weights = vec![1., FRAC_1_SQRT_2, 1.];
    let corner = BSpline::rational(2, vec![0., 0., 0., 1., 1., 1.], poles, weights)?;
    let edges = vec![
        ProfileEdge::Line { a: at(0., 0.), b: at(RADIUS, 0.) },
        ProfileEdge::Line { a: at(RADIUS, 0.), b: at(RADIUS, HEIGHT - DOWN) },
        ProfileEdge::Spline(Arc::new(corner)),
        ProfileEdge::Line { a: at(RADIUS - ACROSS, HEIGHT), b: at(0., HEIGHT) },
        ProfileEdge::Line { a: at(0., HEIGHT), b: at(0., 0.) },
    ];
    let section = Profile { origin: at(0., 0.), normal: [0., -1., 0.],
                            loops: vec![edges], names: Vec::new() };
    revolve(&section, [0., 0., 0.], [0., 0., 1.], TAU)
}
