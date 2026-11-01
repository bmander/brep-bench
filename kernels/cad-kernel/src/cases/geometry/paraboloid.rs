//! The solid paraboloid x² + y² ≤ 8z up to z = 8: the parabola x² = 8z of the xz plane, from its
//! vertex to x = 8, turned once about z.
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::nurbs::BSpline;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;
use std::sync::Arc;

const H: f64 = 8.;
const RIM: f64 = 8.;

pub fn build() -> Result<Brep, String> {
    // a profile has no parabola: its arc as the quadratic Bézier on the tangents at its ends, which
    // meet at (RIM/2, 0), halfway out under the rim
    let (vertex, rim, top) = ([0., 0., 0.], [RIM, 0., H], [0., 0., H]);
    let arc = BSpline::new(2, vec![0., 0., 0., 1., 1., 1.], vec![vertex, [RIM / 2., 0., 0.], rim])?;
    let section = Profile {
        origin: vertex,
        normal: [0., -1., 0.],
        loops: vec![vec![
            ProfileEdge::Spline(Arc::new(arc)),
            ProfileEdge::Line { a: rim, b: top },
            ProfileEdge::Line { a: top, b: vertex },
        ]],
        names: Vec::new(),
    };
    revolve(&section, [0., 0., 0.], [0., 0., 1.], TAU)
}
