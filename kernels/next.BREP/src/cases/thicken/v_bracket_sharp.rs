//! The sheet of two faces meeting at 60° along a crease, the floor from (0, 0, 0) to (10, 8, 0)
//! and the wall through (0, 0, 0), (0, 8, 0), (5, 8, 5√3) and (5, 0, 5√3), thickened by 1 to the
//! outside of the crease, the moved faces extended until they meet: a V 1 thick, sharp.
//!
//! The kernel's thicken of a sheet of several faces (`thicken_sheet_faces`), as
//! `thicken/angle-bracket-sharp` thickens its right-angled bracket. The sheet is the V from
//! (10, 0, 0) by the origin to (5, 0, 5√3) slid 8 along +y into two faces
//! (`extrude_curves_to_sheet`); run that way the faces' normals are −z and (−√3/2, 0, 1/2), to the
//! outside of the crease.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_curves_to_sheet, make_line, thicken_sheet_faces, BrepSolid, OffsetCorners};

pub fn build() -> Result<BrepSolid, Refusal> {
    let top = p(5.0, 0.0, 5.0 * 3f64.sqrt());
    let v = [make_line(p(10.0, 0.0, 0.0), p(0.0, 0.0, 0.0))?, make_line(p(0.0, 0.0, 0.0), top)?];
    let sheet = extrude_curves_to_sheet(&v, p(0.0, 1.0, 0.0), 8.0)?;
    Ok(thicken_sheet_faces(&sheet, 1.0, OffsetCorners::Sharp)?)
}
