//! The block from the origin to (20, 20, 10) with a hole of radius 3 bored through it along z about
//! x = 15.5, y = 10, shelled inward to a wall 1 thick, open at its top: the bore, moved to radius
//! 4, reaches past the inside wall x = 19, and the tube round it merges with that wall.
use crate::solids::p;
use crate::solids::shell::{drilled_block, shell};
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = drilled_block(15.5)?;
    let top = p(2.0, 2.0, 10.0);
    shell(&block, &[top], 1.0)
}
