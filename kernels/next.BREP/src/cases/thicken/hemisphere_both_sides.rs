//! The sheet of the upper half of the sphere of radius 5 about the origin, thickened by 1 to each
//! side: the dome between the spheres of radius 4 and 6 where z ≥ 0.
//!
//! The sheet is the quarter circle from (5, 0, 0) to (0, 0, 5) turned a full turn about the z axis.
use crate::solids::offset::thicken_both;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_arc, make_revolution, BrepSolid};
use std::f64::consts::{FRAC_PI_2, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (o, x, z) = (p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let meridian = make_arc(o, x, z, 5.0, 0.0, FRAC_PI_2)?;
    let sheet = make_revolution(o, z, &meridian, TAU)?;
    thicken_both(&sheet, &[], 2.0)
}
