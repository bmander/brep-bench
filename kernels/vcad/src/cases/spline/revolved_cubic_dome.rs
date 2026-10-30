//! A dome: the region in the xz plane between the z axis from (0, 0, 0) to (0, 0, 10), the x axis
//! from (0, 0, 0) to (9, 0, 0) and the cubic Bézier curve through (9, 0, 0), (12, 0, 4),
//! (9, 0, 10) and (0, 0, 10), turned a full turn about the z axis.
//!
//! Refused: `Solid::revolve` turns only a sketch of line segments (an arc is
//! `SketchError::ArcNotSupported`), and a vcad sketch has no spline segment at all.
//! vcad-kernel-nurbs has the curve (`BSplineCurve`) and a surface of revolution
//! (`NurbsSurface::revolution`), but no operation makes a solid from them.
use vcad_kernel::Solid;

pub fn build() -> Result<Solid, crate::Refusal> {
    Err("vcad revolves only sketches of line segments (`Solid::revolve`; arcs are \
         `ArcNotSupported`) and has no spline in a sketch: no vcad operation turns a Bézier \
         curve into a solid"
        .into())
}
