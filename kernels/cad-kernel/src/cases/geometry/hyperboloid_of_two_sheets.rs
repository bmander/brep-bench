//! The solid inside the upper sheet z²/9 − (x² + y²)/16 = 1 below the plane z = 6: the hyperbola
//! z²/9 − x²/16 = 1 of the xz plane, from its vertex to x = 4√3, turned once about z.
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::nurbs::BSpline;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;
use std::sync::Arc;

const A: f64 = 4.;
const C: f64 = 3.;
const H: f64 = 6.;

pub fn build() -> Result<Brep, String> {
    // a profile has no hyperbola: its arc (A sinh t, C cosh t) for 0 ≤ t ≤ T, cosh T = H/C = 2,
    // as one rational quadratic Bézier, its middle pole where the end tangents meet,
    // (A(sinh T − cosh T tanh(T/2)), C), weighted cosh(T/2)
    let (cosh, sinh) = (H / C, (H * H / (C * C) - 1.).sqrt());
    let half_tanh = sinh / (1. + cosh);
    let weight = ((1. + cosh) / 2.).sqrt();
    let (vertex, rim, top) = ([0., 0., C], [A * sinh, 0., H], [0., 0., H]);
    let middle = [A * (sinh - cosh * half_tanh), 0., C];
    let arc = BSpline::rational(2, vec![0., 0., 0., 1., 1., 1.], vec![vertex, middle, rim],
                                vec![1., weight, 1.])?;
    // out along the arc from the vertex, in along the top, down the axis: counter-clockwise
    // about −y
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
