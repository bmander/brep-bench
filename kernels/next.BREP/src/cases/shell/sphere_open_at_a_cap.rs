//! The part of the ball of radius 5 centred on the origin below the plane z = 3, shelled inward to
//! a wall 1 thick, open at its flat face: the part of that ball less the part of the ball of
//! radius 4 below z = 3, its rim the annulus between radii √7 and 4.
use crate::solids::p;
use crate::solids::shell::{ball_below, shell};
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    let cut = ball_below(3.0)?;
    let flat = p(1.0, 1.0, 3.0);
    shell(&cut, &[flat], 1.0)
}
