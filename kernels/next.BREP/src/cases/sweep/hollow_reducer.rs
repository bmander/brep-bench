//! A hollow square tube tapering along the y axis from y = 0 to y = 40: the outside lofted from the
//! square 24 × 24 at y = 0 to 12 × 12 at y = 40, less the hole lofted from 20 × 20 to 8 × 8, each a
//! two-section loft, whose walls are flat trapezoids. As the Loft feature does with a hole, the
//! hole's loft is cut from the outside's.
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{
    boolean_operation, loft_profile_brep, BooleanOperation, BooleanOptions, BrepSolid, NurbsCurve,
};

/// The square of half-width `h` centred on the y axis at `y`, its sides parallel to x and z.
fn square(y: f64, h: f64) -> Result<Vec<NurbsCurve>, String> {
    polygon(&[p(-h, y, -h), p(h, y, -h), p(h, y, h), p(-h, y, h)])
}

pub fn build() -> Result<BrepSolid, Refusal> {
    let outside = loft_profile_brep(&[square(0.0, 12.0)?, square(40.0, 6.0)?])?;
    let hole = loft_profile_brep(&[square(0.0, 10.0)?, square(40.0, 4.0)?])?;
    Ok(boolean_operation(&outside, &hole, BooleanOperation::Subtract, &BooleanOptions::default())?)
}
