//! The block from the origin to (20, 20, 10) with a hole of radius 3 bored through it along z about
//! x = 10, y = 10, shelled inward to a wall 1 thick, open at its top: a cup with a tube standing in
//! it, the bore moved into the material to radius 4.
use crate::solids::p;
use crate::solids::shell::{drilled_block, shell};
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = drilled_block(10.0)?;
    let top = p(2.0, 2.0, 10.0);
    shell(&block, &[top], 1.0)
}
