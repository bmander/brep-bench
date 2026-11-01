//! Helpers shared by the boolean and tangent cases: the kernel's boolean under its default
//! options, and the rigid motions that place an operand.
#![allow(unused_imports)]
use super::*;
use brep_kernel::{
    boolean_operation, transform_brep, AffineTransform, BooleanOperation, BooleanOptions,
    BrepSolid, KernelRefusal,
};

/// `a ∪ b`.
pub fn union(a: &BrepSolid, b: &BrepSolid) -> Result<BrepSolid, KernelRefusal> {
    boolean_operation(a, b, BooleanOperation::Union, &BooleanOptions::default())
}

/// `a − b`.
pub fn cut(a: &BrepSolid, b: &BrepSolid) -> Result<BrepSolid, KernelRefusal> {
    boolean_operation(a, b, BooleanOperation::Subtract, &BooleanOptions::default())
}

/// `a ∩ b`.
pub fn common(a: &BrepSolid, b: &BrepSolid) -> Result<BrepSolid, KernelRefusal> {
    boolean_operation(a, b, BooleanOperation::Intersect, &BooleanOptions::default())
}

/// `solid` turned by `angle` (radians, right-handed) about the z axis.
pub fn turned_about_z(solid: &BrepSolid, angle: f64) -> Result<BrepSolid, String> {
    let (s, c) = angle.sin_cos();
    #[rustfmt::skip]
    let turn = AffineTransform::new([
        c, -s, 0.0, 0.0,
        s, c, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ])?;
    transform_brep(solid, turn, false)
}
