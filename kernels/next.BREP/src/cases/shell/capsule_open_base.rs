//! A capsule's end, the cylinder of radius 5 on the z axis from z = 0 to z = 10 capped by the half
//! ball of radius 5 about (0, 0, 10), shelled inward to a wall 1 thick, open at its base: the
//! capsule less the capsule of radius 4 from z = 0.
use crate::solids::p;
use crate::solids::shell::{capsule, shell};
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    let end = capsule()?;
    let base = p(1.0, 1.0, 0.0);
    shell(&end, &[base], 1.0)
}
