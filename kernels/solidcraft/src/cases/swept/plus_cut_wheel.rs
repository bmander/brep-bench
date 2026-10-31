//! Refused. The wheel is cut by a plus that turns in mesh with it, as a gear shaper cuts: seen from
//! the wheel, the tool rolls round it, and the faces it leaves are the envelope of the tool's
//! sides over the motion. solidcraft-kernel has no sweep of a solid and no envelope, and its
//! sweeps of a profile only follow a path, a helix or a revolve, none of which is this motion; an
//! envelope computed here would be geometry made outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcraft-kernel has no sweep of a solid and no envelope: the wheel's flanks are the \
         envelope of the plus rolled round it, which no sweep of a profile along a path, a helix \
         or a revolve makes"
        .into())
}
