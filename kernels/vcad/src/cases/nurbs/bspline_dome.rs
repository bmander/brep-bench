//! A block on the square 0 ≤ x, y ≤ 20, its floor at z = 0, its four sides upright, and its top
//! a clamped bicubic B-spline surface of 6 × 5 control points, with a double knot at u = 2/5.
use super::bezier_dome::NO_SPLINE_FACE;

pub fn build() -> Result<vcad_kernel::Solid, crate::Refusal> {
    Err(format!("{NO_SPLINE_FACE}, so not the block under a bicubic B-spline surface").into())
}
