//! The box from the origin to (20, 14, 10) with its four vertical edges rounded to radius 2,
//! shelled inward to a wall 1 thick, open at its top: a tray whose inside corners are rounds of
//! radius 1, concentric with the outside ones.
use crate::solids::p;
use crate::solids::shell::{rounded_plate, shell};
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = rounded_plate()?;
    let top = p(10.0, 7.0, 10.0);
    shell(&plate, &[top], 1.0)
}
