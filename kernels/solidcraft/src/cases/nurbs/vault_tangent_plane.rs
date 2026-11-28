//! The block on the square −12 ≤ x, y ≤ 12 under the weighted barrel vault arching along y, cut
//! by the plane along its crown.
//!
//! On raw solidcore, sewn from its faces as the case gives them: SolidCraft's kernel makes a face
//! on a NURBS net given as data through a T-spline sheet, of odd degree only, and this top is
//! quadratic along y. The top is the rational B-spline of degree 1 along x and 2 along y, knots
//! (0, 0, 1, 1) and (0, 0, 0, 1, 1, 1), its control points (x, −12, 10), (x, 0, 26) and (x, 12, 10)
//! for x = ±12, weighted 1, 3/5 and 1; the sides x = ±12 are bounded by its edge arcs there.
use crate::bench::cut;
use crate::shared::nurbs::{bezier_knots, polygon, rational_curve, rational_spline, segment, sewn, Face};
use crate::Refusal;
use solidcore_ops::{make_box, Body};

/// The control row of the vault's arc in the plane x = `x`.
fn arc(x: f64) -> Vec<[f64; 4]> {
    vec![[x, -12.0, 10.0, 1.0], [x, 0.0, 26.0, 0.6], [x, 12.0, 10.0, 1.0]]
}

pub fn build() -> Result<Body, Refusal> {
    let net = vec![arc(-12.0), arc(12.0)];
    // the side x = `x`: the floor's edge, the two upright edges and the top's edge arc
    let arched = |x: f64| {
        Face::Flat(vec![
            segment([x, -12.0, 0.0], [x, 12.0, 0.0]),
            segment([x, 12.0, 0.0], [x, 12.0, 10.0]),
            rational_curve(&bezier_knots(2), &arc(x)),
            segment([x, -12.0, 10.0], [x, -12.0, 0.0]),
        ])
    };
    let block = sewn(&[
        Face::Surface(rational_spline(&bezier_knots(1), &bezier_knots(2), &net)),
        Face::Flat(polygon(&[[-12.0, -12.0, 0.0], [12.0, -12.0, 0.0], [12.0, 12.0, 0.0], [-12.0, 12.0, 0.0]])),
        arched(-12.0),
        arched(12.0),
        Face::Flat(polygon(&[[-12.0, -12.0, 0.0], [12.0, -12.0, 0.0], [12.0, -12.0, 10.0], [-12.0, -12.0, 10.0]])),
        Face::Flat(polygon(&[[-12.0, 12.0, 0.0], [12.0, 12.0, 0.0], [12.0, 12.0, 10.0], [-12.0, 12.0, 10.0]])),
    ])?;
    // a box whose floor is the plane z = 16, along the vault's crown, reaching past the block on
    // every other side
    cut(block, make_box([-20.0, -20.0, 16.0], [40.0, 40.0, 24.0])?)
}
