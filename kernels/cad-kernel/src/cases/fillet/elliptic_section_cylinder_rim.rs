//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, the circle where its side meets its
//! top rounded unequally, 1 in across the top and 3 down the side, by a quarter ellipse: in each
//! half plane through the axis the quarter of the ellipse about (ρ, z) = (4, 7) with semi-axes 1
//! across and 3 up, tangent to the top at ρ = 4 and to the side at z = 7, the corner outside it
//! removed.
//!
//! cad-kernel's rounds are a ball's (`fillet::roll`, a circular section) or a flat
//! (`fillet::flat_along`); it has no round of two distances, so no quarter ellipse is rolled. The
//! cylinder is a solid of revolution, so it is built as one: its half section with the corner
//! drawn as the quarter ellipse, a rational quadratic on the corner's point weighted √2/2 (the
//! ellipse exactly), turned once about the z axis.
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::nurbs::BSpline;
use cad_kernel::topo::Brep;
use std::f64::consts::{FRAC_1_SQRT_2, TAU};
use std::sync::Arc;

const RADIUS: f64 = 5.;
const HEIGHT: f64 = 10.;
/// The round's legs: across the top and down the side.
const ACROSS: f64 = 1.;
const DOWN: f64 = 3.;

pub fn build() -> Result<Brep, String> {
    // the section in the half plane y = 0, x ≥ 0
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
