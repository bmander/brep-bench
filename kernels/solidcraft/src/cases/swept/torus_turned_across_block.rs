//! Refused. The groove is what a torus, its axis leaning skew to the block's, passes through as it
//! turns about the z axis. Turned about an axis, a solid sweeps the revolve of its shadow in the
//! half-plane through the axis, but the leaning torus's shadow there is bounded by the envelope
//! of its meridian curves, none of the torus's own circles. solidcraft-kernel has no sweep of a
//! solid and no envelope, and that shadow computed here would be geometry made outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcraft-kernel has no sweep of a solid and no envelope: turned about the z axis, the \
         leaning torus sweeps the revolve of its shadow through the axis, whose edge is an \
         envelope and none of the torus's circles"
        .into())
}
