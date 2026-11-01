//! The sheet of the parabolic cylinder z = x²/8, −4 ≤ x ≤ 4, 0 ≤ y ≤ 10, thickened by 1 to its
//! concave side, toward the focus (0, y, 2).
//!
//! The sheet is the B-spline surface the case gives: degree 2 in u and 1 in v, no interior knots,
//! weights 1, control points (−4, y, 2), (0, y, −2), (4, y, 2) in u for y = 0 and 10 in v.
use crate::solids::offset::thicken;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{BrepSolid, NurbsSurface, Vec4};

pub fn build() -> Result<BrepSolid, Refusal> {
    // One control point in u: its two in v, at y = 0 and y = 10.
    let row = |x: f64, z: f64| {
        vec![Vec4::from_point(p(x, 0.0, z), 1.0), Vec4::from_point(p(x, 10.0, z), 1.0)]
    };
    let sheet = NurbsSurface::new(
        2,
        1,
        vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        vec![0.0, 0.0, 1.0, 1.0],
        vec![row(-4.0, 2.0), row(0.0, -2.0), row(4.0, 2.0)],
    )?;
    thicken(&sheet, &[], 1.0, |q| p(-q.x, 0.0, 2.0 - q.z))
}
