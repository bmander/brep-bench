//! The block −12 ≤ x ≤ 12, −8 ≤ y ≤ 8, 0 ≤ z ≤ 8 with the elliptic cylinder x²/36 + y²/9 < 1 cut
//! through it along z. next.BREP has no ellipse among its curves, so the tool is a round cylinder
//! of radius 1 stretched by 6 along x and 3 along y; it runs past both faces.
use crate::solids::geometry::stretch;
use crate::Refusal;
use brep_kernel::{
    boolean_operation, make_box_brep, make_cylinder_brep, transform_brep, BooleanOperation,
    BooleanOptions, BrepSolid, Vec3,
};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(Vec3::new(-12.0, -8.0, 0.0), 24.0, 16.0, 8.0)?;
    let round = make_cylinder_brep(Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 0.0, 1.0), 1.0, 10.0)?;
    let tool = transform_brep(&round, stretch(6.0, 3.0, 1.0)?, false)?;
    Ok(boolean_operation(&block, &tool, BooleanOperation::Subtract, &BooleanOptions::default())?)
}
