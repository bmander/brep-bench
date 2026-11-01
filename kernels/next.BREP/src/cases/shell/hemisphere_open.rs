//! The lower half of the ball of radius 5 centred on the origin, z <= 0, shelled inward to a wall
//! 1 thick, open at its flat face: a bowl, the half ball less the half ball of radius 4.
use crate::solids::p;
use crate::solids::shell::{ball_below, shell};
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    let half = ball_below(0.0)?;
    let flat = p(1.0, 1.0, 0.0);
    shell(&half, &[flat], 1.0)
}
