//! Refused. The groove is what a torus, set at the helix angle, passes through as it is screwed
//! along the rod. The kernel's helical sweep (`sweep_helix`) carries a planar profile along a
//! screw, but no plane section of the torus is the groove's section: the groove's wall is the
//! envelope of the torus over the screw, touching it along a curve that is none of its circles.
//! solidcraft-kernel has no sweep of a solid and no envelope, and that section computed here
//! would be geometry made outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcraft-kernel has no sweep of a solid and no envelope: `sweep_helix` screws a planar \
         profile, but the groove's section is the envelope of the leaning torus over the screw, \
         which is no plane section of it"
        .into())
}
