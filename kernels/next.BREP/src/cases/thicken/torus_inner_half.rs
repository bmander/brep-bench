//! The sheet of the inner half of the torus about the z axis whose tube has radius 2 about a
//! middle circle of radius 6, the points of it no farther than 6 from the axis, thickened by 1
//! away from the middle circle, toward the axis.
//!
//! The sheet is the half circle of radius 2 about (6, 0, 0) from (6, 0, 2) through (4, 0, 0) to
//! (6, 0, −2) turned a full turn about the z axis.
use crate::solids::offset::thicken;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_arc, make_revolution, BrepSolid};
use std::f64::consts::{PI, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let tube = make_arc(p(6.0, 0.0, 0.0), p(0.0, 0.0, 1.0), p(-1.0, 0.0, 0.0), 2.0, 0.0, PI)?;
    let sheet = make_revolution(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), &tube, TAU)?;
    thicken(&sheet, &[], 1.0, |q| {
        let r = q.x.hypot(q.y);
        p(q.x - 6.0 * q.x / r, q.y - 6.0 * q.y / r, q.z)
    })
}
