//! Refused, as `swept/ball-screwed-round-a-tight-helix` is: turning the solid rigidly so that the
//! helix's axis is (1, 2, 2)/3 changes nothing of why. The helix turns tighter than the ball, so
//! the screw sweep of the ball's forward-moving great disc runs through itself, and the solid's
//! boundary is that sweep's envelope trimmed where it meets itself. solidcraft-kernel has no
//! sweep of a solid and no envelope, and that trimmed envelope computed here would be geometry
//! made outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("no layer (solidcraft-kernel, solidcore-ops, solidcore) has a sweep of a solid or an envelope: the helix turns tighter than \
         the ball, so the screw sweep of the ball's forward-moving great disc runs through itself, \
         and the solid's boundary is that sweep's envelope trimmed where it meets itself"
        .into())
}
