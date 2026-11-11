//! A helicon face gear hobbed by its worm.
//!
//! Refused: next.BREP's sweep of a solid takes a prism or a solid of revolution under a screw, a
//! turn or a planar motion; seen from the gear the worm is a rack slid along its axis while turned
//! about the z axis (a planar motion), but its thread is neither a prism along the z axis nor a
//! solid of revolution.
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    Err(Refusal::from(
        "next.BREP's sweep of a solid takes a prism or a solid of revolution under a screw, a \
         turn or a planar motion; seen from the gear the worm is a rack slid along its axis while \
         turned about the z axis (a planar motion), but its thread is neither a prism along the z \
         axis nor a solid of revolution",
    ))
}
