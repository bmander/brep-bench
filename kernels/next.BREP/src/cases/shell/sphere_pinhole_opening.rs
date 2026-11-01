//! The part of the ball of radius 5 centred on the origin below the plane z = 3.9999, shelled
//! inward to a wall 1 thick, open at its flat face: the sphere of radius 4 meets the plane in a
//! pinhole of radius about 0.028, through which the cavity opens.
use crate::solids::p;
use crate::solids::shell::{ball_below, shell};
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    let cut = ball_below(3.9999)?;
    let flat = p(1.0, 1.0, 3.9999);
    shell(&cut, &[flat], 1.0)
}
