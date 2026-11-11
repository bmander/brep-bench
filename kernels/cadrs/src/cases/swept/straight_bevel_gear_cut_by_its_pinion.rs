//! A 30-tooth bevel gear's blank with one tooth space cut by a straight-sided tooth of its
//! 15-tooth pinion, the shafts square and meeting at O: the pinion turns by t about the x axis
//! while the gear turns by −t/2 about the z axis, t from −60° to 60°.
//!
//! The blank is its meridian quadrilateral revolved; the tooth is a pyramid with apex O lofted
//! from its outer trapezoid and cut off at the inner plane, a convex polyhedron. cadrs's sweep of
//! a solid under the two turns (`SolidMotion::Rigid`, seen from the gear: by t about the x axis,
//! then by t/2 about the z axis) builds what the tooth passes through exactly, and it is cut from
//! the blank.
use crate::bench::{p3, v3};
use crate::shared::swept::{bevel_cut, bevel_tooth};
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let tooth = bevel_tooth(k)?;
    bevel_cut(k, tooth, Axis::new(p3(0.0, 0.0, 0.0), v3(1.0, 0.0, 0.0)), -0.5, (-60.0, 60.0))
}
