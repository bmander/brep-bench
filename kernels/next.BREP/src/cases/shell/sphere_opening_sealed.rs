//! The part of the ball of radius 5 centred on the origin below the plane z = 4.5, shelled inward
//! to a wall 1 thick, open at its flat face: the sphere moved in to radius 4 never reaches the
//! plane, so the flat face stays and the cavity is the closed ball of radius 4.
use crate::solids::p;
use crate::solids::shell::{ball_below, shell};
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    let cut = ball_below(4.5)?;
    let flat = p(1.0, 1.0, 4.5);
    shell(&cut, &[flat], 1.0)
}
