//! A hypoid gear's tooth space cut by its pinion.
//!
//! Refused: next.BREP's sweep of a solid takes a prism or a solid of revolution under a screw, a
//! turn or a planar motion; the tooth is a pyramid's frustum, and seen from the gear it turns about
//! two skew axes (the tumbling torus's motion), which it refuses.
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    Err(Refusal::from(
        "next.BREP's sweep of a solid takes a prism or a solid of revolution under a screw, a \
         turn or a planar motion; the tooth is a pyramid's frustum, and seen from the gear it \
         turns about two skew axes (the tumbling torus's motion), which it refuses",
    ))
}
