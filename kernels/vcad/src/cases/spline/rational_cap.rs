//! A ball's cap: the region in the xz plane between the z axis, the segment from (0, 0, −5) to
//! (5√3, 0, −5) and the rational quadratic Bézier arc with control points (5√3, 0, −5),
//! (10√3, 0, 10), (0, 0, 10) and weights 1, ½, 1, turned a full turn about the z axis.
//!
//! Refused: `Solid::revolve` turns only a sketch of line segments (an arc is
//! `SketchError::ArcNotSupported`), and a vcad sketch has no spline segment at all.
//! vcad-kernel-nurbs has the weighted curve (`NurbsCurve`) and its surface of revolution
//! (`NurbsSurface::revolution`), but no operation makes a solid from them.
use vcad_kernel::Solid;

pub fn build() -> Result<Solid, crate::Refusal> {
    Err("vcad revolves only sketches of line segments (`Solid::revolve`; arcs are \
         `ArcNotSupported`) and has no spline in a sketch: no vcad operation turns a rational \
         Bézier arc into a solid"
        .into())
}
