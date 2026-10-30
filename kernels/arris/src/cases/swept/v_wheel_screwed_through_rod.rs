//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a helical V groove
//! ground round it by a wheel with a sharp 60° V rim (its axis through (14, 0, 0), leaning by the
//! helix angle) carried along a screw about the z axis that rises 30 a turn, from −144° to +384°.
//!
//! Refused: arris has no sweep of a solid, and none along a screw motion. The groove's flanks are
//! the envelopes of the wheel's two conical flanks screwed, each swept along the screw from the
//! curve where the screw's velocity is tangent to the cone, and its root the helix-like trace of
//! the rim's edge; none is a surface arris builds from a profile, a path or a revolution, so the
//! groove could only be made by computing those characteristic curves outside the kernel.
use crate::Refusal;
use arris::topo::{Body, Model};

pub fn build(_: &mut Model) -> Result<Body, Refusal> {
    Err("arris has no sweep of a solid along a screw motion; the groove's flanks are envelopes of \
         the screwed V wheel's cones, which it cannot build"
        .into())
}
