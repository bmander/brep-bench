//! A thin blade from z = 0 to z = 60: the rectangle 40 × 0.4 centred on the z axis, turned about it
//! right-handed by 6° for every unit of height, a whole turn in all, its corners on four helices.
//!
//! next.BREP's loft takes no guide curves for its corners, but the section is one rectangle turned
//! steadily as it rises, which is the kernel's twisted sweep (`sweep_profile_twisted`): the
//! rectangle at z = 0 swept up the z axis, turning a whole turn about it in proportion to the
//! height risen.
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{make_line, sweep_profile_twisted, BrepSolid};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, Refusal> {
    let foot = polygon(&[p(20.0, -0.2, 0.0), p(20.0, 0.2, 0.0), p(-20.0, 0.2, 0.0),
        p(-20.0, -0.2, 0.0)])?;
    let axis = make_line(p(0.0, 0.0, 0.0), p(0.0, 0.0, 60.0))?;
    Ok(sweep_profile_twisted(&foot, &axis, TAU, None)?)
}
