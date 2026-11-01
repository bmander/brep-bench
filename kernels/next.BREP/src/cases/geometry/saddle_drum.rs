//! A drum of radius 8 about the z axis, its floor at z = 0 and its top the saddle z = 10 + xy/8,
//! made as the bilinear B-spline patch on (−10, −10, 22.5), (10, −10, −2.5), (−10, 10, −2.5) and
//! (10, 10, 22.5).
//!
//! The block under the whole patch, down to z = −5, is lofted from its end at x = −10 to its end
//! at x = 10: two ruled sections, so its top is the patch ruled between its edges at x = ±10, the
//! bilinear patch on those four corners. The drum is that block in common with the cylinder of
//! radius 8 from the floor up past the saddle, which trims the patch.
use crate::solids::geometry::under_saddle;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{
    boolean_operation, make_cylinder_brep, BooleanOperation, BooleanOptions, BrepSolid,
};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = under_saddle()?;
    let drum = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 8.0, 30.0)?;
    Ok(boolean_operation(&block, &drum, BooleanOperation::Intersect, &BooleanOptions::default())?)
}
