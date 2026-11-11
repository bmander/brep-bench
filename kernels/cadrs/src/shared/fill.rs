//! What the cadrs implementations of the fill cases share: Fill (Onshape's Fill) through sketch
//! curves, each on its own plane, and the flat walls under a patch's boundary curves.
//!
//! cadrs's Fill fills a planar loop with its plane and four curves with their bilinearly blended
//! Coons patch (sampled and interpolated by a bicubic B-spline, exact for boundaries of degree two
//! or less, whose patch is of degree two or less each way).
use crate::Refusal;
use cadrs_kernel::{BodyId, Curve2, DefaultKernel, FillSpec, Kernel, Plane};

/// The sheet Fill makes of the closed chain `curves` (each a curve sketched on its plane).
pub fn fill(k: &mut DefaultKernel, curves: &[(Plane, Curve2)]) -> Result<BodyId, Refusal> {
    Ok(k.fill(&FillSpec::sketched(curves))?.first()?)
}

/// The sketch segment from `a` to `b`.
pub fn seg(a: (f64, f64), b: (f64, f64)) -> Curve2 {
    Curve2::line([a.0, a.1], [b.0, b.1])
}

/// The polynomial Bézier curve with these control points: a sketch spline of one span.
pub fn bezier(poles: &[(f64, f64)]) -> Curve2 {
    Curve2::bezier(&poles.iter().map(|&(x, y)| [x, y]).collect::<Vec<_>>())
}

/// The flat wall on `plane` under its curve `top` (from `a` to `b`, each a sketch point (s, h)
/// with h its height): down from `b` to the floor h = 0, along it back under `a`, and up to `a`.
pub fn wall(
    k: &mut DefaultKernel,
    plane: Plane,
    top: Curve2,
    a: (f64, f64),
    b: (f64, f64),
) -> Result<BodyId, Refusal> {
    fill(
        k,
        &[
            (plane, top),
            (plane, seg(b, (b.0, 0.0))),
            (plane, seg((b.0, 0.0), (a.0, 0.0))),
            (plane, seg((a.0, 0.0), a)),
        ],
    )
}
