//! A ball's cap: the region in the xz plane between the z axis from (0, 0, −5) to (0, 0, 10), the
//! segment from (0, 0, −5) to (5√3, 0, −5) and the rational quadratic Bézier arc through `CONTROL`
//! with weights `WEIGHTS`, turned a full turn about z.
//!
//! Built as `spline/revolved-cubic-dome` is: the weighted arc turned into its exact NURBS surface
//! of revolution, the dome and its base assembled round it and finished by `ops::build`.
use super::revolved_cubic_dome::turned;
use crate::Refusal;
use arris::geom::NurbsCurve;
use arris::math::Point3;
use arris::topo::{Body, Model};

/// √3.
const ROOT3: f64 = 1.732_050_807_568_877_2;
const CONTROL: [[f64; 3]; 3] =
    [[5.0 * ROOT3, 0.0, -5.0], [10.0 * ROOT3, 0.0, 10.0], [0.0, 0.0, 10.0]];
const WEIGHTS: [f64; 3] = [1.0, 0.5, 1.0];

pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    let points = CONTROL.iter().map(|&[x, y, z]| Point3::new(x, y, z)).collect();
    let arc = NurbsCurve::new(2, vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], points, WEIGHTS.to_vec())
        .map_err(|e| e.to_string())?;
    turned(m, arc)
}
