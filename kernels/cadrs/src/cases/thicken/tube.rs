//! A sheet: the whole cylinder of radius 5 about the z axis from z = 0 to z = 10, one face closing
//! round on itself, bounded only by its two circles, thickened by 1 away from the axis.
//!
//! The circle of radius 5 about the origin sketched on the Top plane and surface-extruded 10 along
//! +z (the sheet its curve sweeps, no caps); cadrs's Thicken thickens it with its arrow flipped to
//! point away from the axis.
use crate::bench::{p3, v3};
use crate::shared::thicken::{surface_extrude, thicken_toward};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let s = Profile::of(Plane::xy(0.0), vec![Loop::circle([0.0, 0.0], 5.0)]);
    let sheet = surface_extrude(k, &s, v3(0.0, 0.0, 1.0), 10.0)?;
    thicken_toward(k, sheet, p3(0.0, 5.0, 5.0), v3(0.0, 1.0, 0.0), 1.0)
}
