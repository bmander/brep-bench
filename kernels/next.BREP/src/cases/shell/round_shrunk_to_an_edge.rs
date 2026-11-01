//! The box from the origin to (20, 14, 10) with its four vertical edges rounded to radius 2,
//! shelled inward to a wall 2 thick, open at its top: each round, moved 2 inward, shrinks to its
//! axis, and the cavity is the sharp-cornered box from (2, 2, 2) to (18, 12, 10).
use crate::solids::p;
use crate::solids::shell::{rounded_plate, shell};
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = rounded_plate()?;
    let top = p(10.0, 7.0, 10.0);
    shell(&plate, &[top], 2.0)
}
