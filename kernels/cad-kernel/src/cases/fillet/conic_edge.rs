//! The box from the origin to (20, 10, 10), the edge where its top z = 10 meets its end x = 20
//! blended by a conic of rho 0.5 (a parabola) tangent to the top and the end each 3 from the edge:
//! in every plane y = c the quadratic Bézier through (17, c, 10), (20, c, 10) and (20, c, 7).
//!
//! Built round the kernel's lack of a conic blend: cad-kernel blends an edge only by a ball
//! (`fillet::roll`, a circular section) or a flat (`fillet::flat_along`, a chamfer), and the
//! corners of a section (`corners`) only by a ball, a flat or a chamfer's sum of distances. So the
//! piece the blend takes off is drawn from what the case says it is: in each plane y = c the
//! region between the edge's two legs and the conic, the rational quadratic Bézier on the legs'
//! ends with the edge's point between them weighed rho / (1 − rho) — a profile edge the kernel
//! holds exactly — swept along the edge and cut from the block.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{prism, Profile, ProfileEdge};
use cad_kernel::nurbs::BSpline;
use cad_kernel::topo::Brep;
use std::sync::Arc;

const LENGTH: f64 = 20.;
const WIDTH: f64 = 10.;
const HEIGHT: f64 = 10.;
/// How far the blend runs back along the top and down the end from the edge.
const LEGS: [f64; 2] = [3., 3.];
const RHO: f64 = 0.5;

pub fn build() -> Result<Brep, String> {
    let solid = block([0., 0., 0.], [LENGTH, WIDTH, HEIGHT])?;
    // the edge's point in the plane y = 0, and where the blend touches the top and the end
    let edge = [LENGTH, 0., HEIGHT];
    let (top, end) = ([LENGTH - LEGS[0], 0., HEIGHT], [LENGTH, 0., HEIGHT - LEGS[1]]);
    let w = RHO / (1. - RHO);
    let (knots, weights) = (vec![0., 0., 0., 1., 1., 1.], (w != 1.).then(|| vec![1., w, 1.]));
    let conic = BSpline { degree: 2, knots, poles: vec![top, edge, end], weights };
    // the piece's section: the two legs and the conic between their ends, swept along +y
    let section = Profile { origin: edge, normal: [0., 1., 0.], names: vec![], loops: vec![vec![
        ProfileEdge::Line { a: top, b: edge },
        ProfileEdge::Line { a: edge, b: end },
        ProfileEdge::Spline(Arc::new(conic)),
    ]] };
    let piece = prism(&section, 0., WIDTH)?;
    Ok(combined(&solid, &piece, Op::Cut, 0.)?)
}
