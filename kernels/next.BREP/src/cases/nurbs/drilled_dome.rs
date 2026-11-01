//! The block under the bicubic Bézier dome of `nurbs/bezier-dome`, with a round hole of radius 3
//! drilled straight down through it about the line x = 12, y = 8: the block less a cylinder that
//! runs past the floor and past the top.
use crate::solids::p;
use crate::solids::sweep::block_under;
use crate::Refusal;
use brep_kernel::{
    boolean_operation, make_cylinder_brep, BooleanOperation, BooleanOptions, BrepSolid,
};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = block_under(&super::bezier_dome::dome()?)?;
    let drill = make_cylinder_brep(p(12.0, 8.0, -5.0), p(0.0, 0.0, 1.0), 3.0, 30.0)?;
    Ok(boolean_operation(&block, &drill, BooleanOperation::Subtract, &BooleanOptions::default())?)
}
