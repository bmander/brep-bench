//! The block −12 ≤ x ≤ 12, −8 ≤ y ≤ 8, 0 ≤ z ≤ 8 with the elliptic cylinder x²/36 + y²/9 < 1 cut
//! through it along z. The tool is the kernel's ellipse (`make_ellipse`), semi-axes 6 along x and
//! 3 along y, extruded from below the block to above it.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{
    boolean_operation, extrude_profile_brep, make_box_brep, make_ellipse, BooleanOperation,
    BooleanOptions, BrepSolid,
};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (x, y, z) = (p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0), p(0.0, 0.0, 1.0));
    let block = make_box_brep(p(-12.0, -8.0, 0.0), 24.0, 16.0, 8.0)?;
    let tool = extrude_profile_brep(&[make_ellipse(p(0.0, 0.0, -1.0), x, y, 6.0, 3.0)?], z, 10.0)?;
    Ok(boolean_operation(&block, &tool, BooleanOperation::Subtract, &BooleanOptions::default())?)
}
