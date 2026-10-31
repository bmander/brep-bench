//! Refused. The tooth space is cut by a rack tooth rolled round the blank: seen from the blank,
//! the tooth turns as it slides, and the space's flanks (involutes) and root are the envelope of
//! the tooth's sides and tip corners over the motion. solidcraft-kernel has no sweep of a solid
//! and no envelope, and its sweeps of a profile only follow a path, a helix or a revolve, none of
//! which is this rolling; an envelope computed here would be geometry made outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcraft-kernel has no sweep of a solid and no envelope: the tooth space is the \
         envelope of the rack tooth rolled round the blank, which no sweep of a profile along a \
         path, a helix or a revolve makes"
        .into())
}
