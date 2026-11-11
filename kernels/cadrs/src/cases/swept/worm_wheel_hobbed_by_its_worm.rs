//! A worm wheel, the disk of radius 32 about the z axis from z = −6 to 6, hobbed whole by its
//! single-start worm (lead 2π, pitch radius 10) on the line through (0, 40, 0) along x, from
//! x = −20 to 20: the worm turns by t about its axis while the wheel turns by t/30, t from 0 to
//! 60π.
//!
//! The thread is its axial trapezoid swept along a helix (cadrs's sweep, by the helix's screw
//! motion), cut to its ends and placed; cadrs's cut by a moving tool under the two turns
//! (`Kernel::cut_swept`, `SolidMotion::Rigid`): each tooth space two flanks the thread's flanks
//! envelop, the root its crest envelops and the two fillets its crest's edges trace.
use crate::bench::{cylinder, p3, v3};
use crate::shared::swept::hobbed;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let wheel = cylinder(k, p3(0.0, 0.0, -6.0), v3(0.0, 0.0, 1.0), 32.0, 12.0)?;
    hobbed(k, wheel, p3(0.0, 40.0, 0.0), -20.0, 20.0)
}
