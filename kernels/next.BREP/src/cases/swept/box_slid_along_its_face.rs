//! The box from (0, 0, 0) to (10, 6, 4) slid without turning along (8, 5, 0): the kernel's sweep
//! of a solid, as `swept/box-slid-aslant` slides its box along (8, 5, 3).
use crate::solids::p;
use crate::solids::swept::{box_tool, slide_by};
use crate::Refusal;
use brep_kernel::{sweep_solid, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let tool = box_tool(p(0.0, 0.0, 0.0), p(10.0, 0.0, 0.0), p(0.0, 6.0, 0.0), p(0.0, 0.0, 4.0));
    Ok(sweep_solid(&tool, &slide_by(p(8.0, 5.0, 0.0)))?)
}
