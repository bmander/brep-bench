//! A face gear, the ring between radii 40 and 52 about the z axis from z = −6 to 2, hobbed whole by
//! a single-start worm (lead 2π, pitch radius 10) on the line through (0, 30, 10) along x, from
//! x = 0 to 52: the worm turns by t about its axis while the gear turns by t/30, t from 0 to 60π.
//!
//! The ring is a circle and its bore extruded; the thread its axial trapezoid swept along a helix
//! (cadrs's sweep, by the helix's screw motion), cut to its ends and placed; cadrs's cut by a
//! moving tool under the two turns (`Kernel::cut_swept`, `SolidMotion::Rigid`): each tooth space
//! the sheets the thread's flank and crest envelop and the ones its crest's edges trace.
use crate::bench::{extrude, p3};
use crate::shared::swept::hobbed;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let face = Profile::with_holes(Plane::xy(-6.0), Loop::circle([0.0, 0.0], 52.0), vec![Loop::circle([0.0, 0.0], 40.0)]);
    let ring = extrude(k, &face, 8.0)?;
    hobbed(k, ring, p3(0.0, 30.0, 10.0), 0.0, 52.0)
}
