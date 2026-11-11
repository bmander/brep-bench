//! A straight bevel gear's tooth space cut by its pinion.
//!
//! Refused: next.BREP's sweep of a solid takes a prism or a solid of revolution under a screw, a
//! turn or a planar motion; the tooth is a pyramid's frustum, and seen from the gear it turns about
//! two axes that meet, a spherical motion in none of those classes.
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    Err(Refusal::from(
        "next.BREP's sweep of a solid takes a prism or a solid of revolution under a screw, a \
         turn or a planar motion; the tooth is a pyramid's frustum, and seen from the gear it \
         turns about two axes that meet, a spherical motion in none of those classes",
    ))
}
