//! A prism 10 tall, from z = 0 to z = 10, on the peanut inside the closed uniform periodic cubic
//! B-spline through twelve control points in the plane z = 0.
//!
//! Refused: a vcad sketch holds only lines and circular arcs (`SketchSegment::{Line, Arc}`), and
//! `Solid::extrude` extrudes only a sketch. vcad-kernel-nurbs has B-spline curves, and a surface
//! `NurbsSurface::extrusion` of one, but no operation makes a solid from a spline section.
use vcad_kernel::Solid;

pub fn build() -> Result<Solid, crate::Refusal> {
    Err("vcad has no spline in a sketch: `SketchSegment` is a line or a circular arc, and \
         `Solid::extrude` extrudes only a sketch; no vcad operation makes a solid from a B-spline \
         section"
        .into())
}
