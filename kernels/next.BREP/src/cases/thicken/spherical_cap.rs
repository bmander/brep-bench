//! The sheet of the cap of the sphere of radius 10 about the origin where z ≥ 6, thickened by 2
//! away from the centre: the solid between the spheres of radius 10 and 12 inside the cone about
//! the z axis, apex at the origin, through the cap's rim.
//!
//! The sheet is the arc of the circle of radius 10 from (8, 0, 6) to (0, 0, 10) turned a full turn
//! about the z axis.
use crate::solids::offset::thicken;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_arc, make_revolution, BrepSolid};
use std::f64::consts::{FRAC_PI_2, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (o, x, z) = (p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let meridian = make_arc(o, x, z, 10.0, 6f64.atan2(8.0), FRAC_PI_2)?;
    let sheet = make_revolution(o, z, &meridian, TAU)?;
    thicken(&sheet, &[], 2.0, |q| q)
}
