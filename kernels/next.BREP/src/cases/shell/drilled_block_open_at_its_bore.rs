//! The block from the origin to (20, 20, 10) with a hole of radius 3 bored through it along z about
//! x = 10, y = 10, shelled inward to a wall 1 thick, open at the bore: a hollow box, the cavity
//! from (1, 1, 1) to (19, 19, 9) opening into the hole.
use crate::solids::p;
use crate::solids::shell::{drilled_block, shell};
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = drilled_block(10.0)?;
    // a point of the bore, on the cylinder of radius 3 about x = 10, y = 10
    let bore = p(10.0 - 1.8, 10.0 - 2.4, 5.0);
    shell(&block, &[bore], 1.0)
}
