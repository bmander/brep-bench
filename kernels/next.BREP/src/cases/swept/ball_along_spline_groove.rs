//! The box from (−6, −6, −8) to (36, 26, 0), less everything a ball of radius 3 passes through as
//! its centre follows the cubic Bézier curve in its top face with control points (0, 0, 0),
//! (10, 0, 0), (20, 20, 0) and (30, 20, 0).
//!
//! next.BREP has no sweep of a solid. A ball's sweep is, by elementary geometry, the tube of the
//! ball's radius about its centre's path with a ball at each end: the tube is the kernel's path
//! sweep of the ball's great circle square to the curve, and the balls are its spheres; their
//! union is cut from the block.
use crate::solids::p;
use crate::solids::sweep::{bezier, circle};
use crate::Refusal;
use brep_kernel::{
    boolean_operation, boolean_operation_nary, make_box_brep, make_sphere_brep,
    sweep_profile_along_path, BooleanOperation, BooleanOptions, BrepSolid,
};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(-6.0, -6.0, -8.0), 42.0, 32.0, 8.0)?;
    let path = bezier(&[p(0.0, 0.0, 0.0), p(10.0, 0.0, 0.0), p(20.0, 20.0, 0.0),
        p(30.0, 20.0, 0.0)])?;
    // the ball's circle square to the curve where it starts, along (1, 0, 0)
    let tube = sweep_profile_along_path(&circle(p(0.0, 0.0, 0.0), p(0.0, 1.0, 0.0),
        p(0.0, 0.0, 1.0), 3.0)?, &path, None)?;
    let up = p(0.0, 0.0, 1.0);
    let start = make_sphere_brep(p(0.0, 0.0, 0.0), 3.0, up)?;
    let end = make_sphere_brep(p(30.0, 20.0, 0.0), 3.0, up)?;
    let swept = boolean_operation_nary(&[tube, start, end], BooleanOperation::Union)?;
    Ok(boolean_operation(&block, &swept, BooleanOperation::Subtract, &BooleanOptions::default())?)
}
