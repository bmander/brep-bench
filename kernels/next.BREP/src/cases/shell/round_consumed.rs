//! The box from the origin to (20, 14, 10) with its four vertical edges rounded to radius 2,
//! shelled inward to a wall 3 thick, open at its top: the rounds, moved 3 inward, are consumed,
//! and the walls beside them run on to sharp edges, the cavity the box from (3, 3, 3) to
//! (17, 11, 10).
use crate::solids::p;
use crate::solids::shell::{rounded_plate, shell};
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = rounded_plate()?;
    let top = p(10.0, 7.0, 10.0);
    shell(&plate, &[top], 3.0)
}
