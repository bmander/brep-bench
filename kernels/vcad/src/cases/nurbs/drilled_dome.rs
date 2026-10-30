//! The block under the bicubic Bézier dome of `nurbs/bezier-dome`, drilled straight down through
//! by the vertical hole of radius 3 about the line x = 12, y = 8.
use super::bezier_dome::NO_SPLINE_FACE;

pub fn build() -> Result<vcad_kernel::Solid, crate::Refusal> {
    Err(format!("{NO_SPLINE_FACE}, so not the block under a bicubic Bézier patch to drill").into())
}
