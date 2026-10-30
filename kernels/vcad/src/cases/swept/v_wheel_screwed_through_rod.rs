//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a helical V groove
//! ground round it by a wheel with a sharp 60° V rim (its axis through (14, 0, 0), leaning by the
//! helix angle) carried along a screw about the z axis that rises 30 a turn, from −144° to +384°.
//!
//! Refused: vcad has no sweep of a solid, and none along a screw motion (`Solid::sweep` moves a
//! sketch profile along a curve, a `Helix` among them). The groove's flanks are the envelopes of
//! the wheel's two conical flanks screwed, each swept along the screw from the curve where the
//! screw's velocity is tangent to the cone, and its root the trace of the rim's edge; their
//! section is not the wheel's, so the groove could only be made by computing those characteristic
//! curves outside the kernel.
use vcad_kernel::Solid;

pub fn build() -> Result<Solid, crate::Refusal> {
    Err("vcad has no sweep of a solid along a screw motion; the groove's flanks are envelopes of \
         the wheel's conical flanks screwed, which would have to be computed outside the kernel"
        .into())
}
