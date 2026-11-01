//! A capsule's end, the cylinder of radius 5 on the z axis from z = 0 to z = 10 capped by the half
//! ball of radius 5 about (0, 0, 10), shelled inward to a wall 1 thick, open at its hemisphere:
//! the side, moved in to radius 4, runs on up into the dome until it meets the sphere at z = 13.
use crate::solids::p;
use crate::solids::shell::{capsule, shell};
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    let end = capsule()?;
    // a point of the dome, on the sphere of radius 5 about (0, 0, 10), off its seam
    let dome = p(1.8, 2.4, 14.0);
    shell(&end, &[dome], 1.0)
}
