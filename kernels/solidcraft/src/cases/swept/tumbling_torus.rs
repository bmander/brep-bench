//! Refused. The torus moves by two turns about skew axes at once, so no line of it stays fixed and
//! its sweep's boundary is an envelope of no simpler kind. solidcraft-kernel has no sweep of a
//! solid and no envelope, and an envelope computed here would be geometry made outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcraft-kernel has no sweep of a solid and no envelope: the tumbling torus's sweep is \
         bounded by the envelope of its places, which no sweep of a profile makes"
        .into())
}
