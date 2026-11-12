//! The ellipsoid x²/36 + y²/16 + z²/9 ≤ 1 with everything below z = 0 cut away by a block. The
//! ellipsoid is the kernel's (`make_ellipsoid_brep`, semi-axes 6, 4 and 3 along x, y and z); the
//! block reaches past it everywhere but at z = 0.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{
    boolean_operation, make_box_brep, make_ellipsoid_brep, BooleanOperation, BooleanOptions,
    BrepSolid,
};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (o, x, y) = (p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0));
    let ellipsoid = make_ellipsoid_brep(o, x, y, [6.0, 4.0, 3.0])?;
    let below = make_box_brep(p(-10.0, -10.0, -10.0), 20.0, 20.0, 10.0)?;
    let options = BooleanOptions::default();
    Ok(boolean_operation(&ellipsoid, &below, BooleanOperation::Subtract, &options)?)
}
