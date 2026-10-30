//! A block on the square 0 ≤ x, y ≤ 20, its floor at z = 0, its four sides upright, and its top
//! the bicubic Bézier patch over an even 4 × 4 grid of the square at the heights zᵢⱼ.

/// Why vcad cannot make a solid with a face on a given spline surface.
pub const NO_SPLINE_FACE: &str = "vcad has no operation that makes a face on a B-spline surface \
    given by its control points: its B-spline and NURBS surfaces (vcad-kernel-nurbs's \
    BSplineSurface and NurbsSurface) are not part of vcad-kernel's API, sketches hold only lines \
    and arcs, loft is ruled with planar faces (its Smooth mode falls back to ruled), and only STEP \
    import (and the rolling-ball fillet's own blends) makes B-spline faces";

pub fn build() -> Result<vcad_kernel::Solid, crate::Refusal> {
    Err(format!("{NO_SPLINE_FACE}, so not the block under a bicubic Bézier patch").into())
}
