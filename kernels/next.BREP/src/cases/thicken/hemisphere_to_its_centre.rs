//! The sheet of the upper half of the sphere of radius 5 about the origin, thickened by 5 toward
//! the centre, its whole radius: the half ball of radius 5 where z ≥ 0.
//!
//! The sheet is the quarter circle from (5, 0, 0) to (0, 0, 5) turned a full turn about the z axis.
use crate::solids::offset::thicken;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_arc, make_revolution, BrepSolid};
use std::f64::consts::{FRAC_PI_2, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (o, x, z) = (p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let meridian = make_arc(o, x, z, 5.0, 0.0, FRAC_PI_2)?;
    let sheet = make_revolution(o, z, &meridian, TAU)?;
    thicken(&sheet, &[], 5.0, |q| p(-q.x, -q.y, -q.z))
}
