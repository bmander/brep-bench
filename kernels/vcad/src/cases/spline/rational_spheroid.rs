//! A prolate spheroid: the region in the xz plane between the z axis and the rational quadratic
//! B-spline through (0, 0, −10), (6, 0, −10), (6, 0, 0), (6, 0, 10), (0, 0, 10), weights 1, √2/2,
//! 1, √2/2, 1 and knots (0, 0, 0, ½, ½, 1, 1, 1), turned a full turn about the z axis.
//!
//! Refused: `Solid::revolve` turns only a sketch of line segments (an arc is
//! `SketchError::ArcNotSupported`), and a vcad sketch has no spline segment at all.
//! vcad-kernel-nurbs has the weighted curve (`NurbsCurve`) and its surface of revolution
//! (`NurbsSurface::revolution`), but no operation makes a solid from them; nor may a profile vertex
//! lie on the axis (`SketchError::AxisIntersection`), where this curve meets it at both ends.
use vcad_kernel::Solid;

pub fn build() -> Result<Solid, crate::Refusal> {
    Err("vcad revolves only sketches of line segments (`Solid::revolve`; arcs are \
         `ArcNotSupported`) with no vertex on the axis, and has no spline in a sketch: no vcad \
         operation turns a rational B-spline into a solid"
        .into())
}
