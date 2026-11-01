//! The ellipsoid x²/36 + y²/16 + z²/9 ≤ 1 with everything below z = 0 cut away by a block. The
//! ellipsoid is the ball of radius 1 stretched by 6, 4 and 3 (next.BREP has no ellipsoid); the
//! block reaches past it everywhere but at z = 0.
use crate::solids::geometry::stretch;
use crate::Refusal;
use brep_kernel::{
    boolean_operation, make_box_brep, make_sphere_brep, transform_brep, BooleanOperation,
    BooleanOptions, BrepSolid, Vec3,
};

pub fn build() -> Result<BrepSolid, Refusal> {
    let ball = make_sphere_brep(Vec3::new(0.0, 0.0, 0.0), 1.0, Vec3::new(0.0, 0.0, 1.0))?;
    let ellipsoid = transform_brep(&ball, stretch(6.0, 4.0, 3.0)?, false)?;
    let below = make_box_brep(Vec3::new(-10.0, -10.0, -10.0), 20.0, 20.0, 10.0)?;
    let options = BooleanOptions::default();
    Ok(boolean_operation(&ellipsoid, &below, BooleanOperation::Subtract, &options)?)
}
