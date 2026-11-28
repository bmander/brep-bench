//! Refused, as `swept/torus-screwed-through-rod` is: turning the rod and the wheel rigidly so
//! that the axis is (1, 2, 2)/3 changes nothing of why. The kernel's helical sweep
//! (`sweep_helix`) carries a planar profile along a screw, but no plane section of the torus is
//! the groove's section: the groove's wall is the envelope of the torus over the screw, touching
//! it along a curve that is none of its circles. solidcraft-kernel has no sweep of a solid and no
//! envelope, and that section computed here would be geometry made outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("no layer (solidcraft-kernel, solidcore-ops, solidcore) has a sweep of a solid or an envelope: no layer's screw sweep (`sweep_helix`, `screw_sweep`) does more than screw a planar \
         profile, but the groove's section is the envelope of the leaning torus over the screw, \
         which is no plane section of it"
        .into())
}
