//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, the circle where its side meets its
//! top blended by a conic of rho 0.5, a parabola, tangent to the top and to the side each 2 from
//! the edge: in each half plane through the axis the quadratic Bézier curve on (ρ, z) = (3, 10),
//! (5, 10) and (5, 8), the corner outside it removed.
//!
//! cad-kernel blends an edge only by a ball (`fillet::roll`, a circular section) or a flat
//! (`fillet::flat_along`); it has no blend whose section is a conic (see `conic_edge`). The
//! cylinder is a solid of revolution, so it is built as one: its half section with the corner
//! drawn as the parabola, the quadratic Bézier itself, turned once about the z axis.
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::nurbs::BSpline;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;
use std::sync::Arc;

const RADIUS: f64 = 5.;
const HEIGHT: f64 = 10.;
/// How far the blend runs along the top and down the side from the edge.
const LEG: f64 = 2.;

pub fn build() -> Result<Brep, String> {
    // the section in the half plane y = 0, x ≥ 0; a conic of rho 0.5 is the non-rational quadratic
    // on the two tangent points and the edge's point between them
    let at = |rho: f64, z: f64| [rho, 0., z];
    let poles = vec![at(RADIUS, HEIGHT - LEG), at(RADIUS, HEIGHT), at(RADIUS - LEG, HEIGHT)];
    let corner = BSpline::new(2, vec![0., 0., 0., 1., 1., 1.], poles)?;
    let edges = vec![
        ProfileEdge::Line { a: at(0., 0.), b: at(RADIUS, 0.) },
        ProfileEdge::Line { a: at(RADIUS, 0.), b: at(RADIUS, HEIGHT - LEG) },
        ProfileEdge::Spline(Arc::new(corner)),
        ProfileEdge::Line { a: at(RADIUS - LEG, HEIGHT), b: at(0., HEIGHT) },
        ProfileEdge::Line { a: at(0., HEIGHT), b: at(0., 0.) },
    ];
    let section = Profile { origin: at(0., 0.), normal: [0., -1., 0.],
                            loops: vec![edges], names: Vec::new() };
    revolve(&section, [0., 0., 0.], [0., 0., 1.], TAU)
}
