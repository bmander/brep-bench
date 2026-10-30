//! The block under the bicubic Bézier dome of `nurbs/bezier-dome`, less everything above the
//! plane z = 4 + x/2, which passes through its corner (0, 0, 4).
use super::bezier_dome::NO_SPLINE_FACE;

pub fn build() -> Result<vcad_kernel::Solid, crate::Refusal> {
    Err(format!("{NO_SPLINE_FACE}, so not the block under a bicubic Bézier patch to slice").into())
}
