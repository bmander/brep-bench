//! A spiroid face gear hobbed by its tapered worm.
//!
//! Refused: next.BREP's sweep of a solid takes a prism or a solid of revolution under a screw, a
//! turn or a planar motion; the worm's thread is neither, and seen from the gear the tapered worm
//! turns about two skew axes, which it refuses.
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    Err(Refusal::from(
        "next.BREP's sweep of a solid takes a prism or a solid of revolution under a screw, a \
         turn or a planar motion; the worm's thread is neither, and seen from the gear the \
         tapered worm turns about two skew axes, which it refuses",
    ))
}
