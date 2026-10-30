//! The solid a torus (centred at (3, 0, 2), axis along z, radii 1 and 0.5) fills as it turns by
//! t/4 about the vertical line through (2, 0, 0) while seen from a frame turning by t about the
//! line through (0, 0.5, 0.5) along x, for t from −75° to 75°.
//!
//! Refused: vcad has no sweep of a solid. The motion keeps no line of the torus fixed, so the
//! sweep is neither a revolution nor a slide of it, and its boundary is the envelope of the
//! torus's places, the curves on it where the motion's velocity is tangent to it; vcad has no
//! operation that builds that surface, and it would have to be computed outside the kernel.
use vcad_kernel::Solid;

pub fn build() -> Result<Solid, crate::Refusal> {
    Err("vcad has no sweep of a solid along a motion; the tumbling torus's boundary is its \
         envelope, which would have to be computed outside the kernel"
        .into())
}
