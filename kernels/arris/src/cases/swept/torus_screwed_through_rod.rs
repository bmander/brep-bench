//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a helical groove
//! ground round it by a torus (radii 6 and 2, centred at (11.5, 0, 0), its axis leaning by the
//! helix angle) carried along a screw about the z axis that rises 30 a turn, from −108° to +348°.
//!
//! Refused: arris has no sweep of a solid, and none along a screw motion. The groove's wall is the
//! envelope of the screwed torus, the curve on it where the screw's velocity is tangent to it
//! swept along the screw; its section is none of the torus's own circles, and arris has no
//! operation that builds it. Building it would mean computing that characteristic curve outside
//! the kernel and fitting a surface to it.
use crate::Refusal;
use arris::topo::{Body, Model};

pub fn build(_: &mut Model) -> Result<Body, Refusal> {
    Err("arris has no sweep of a solid along a screw motion; the groove's wall is the envelope of \
         the screwed torus, which it cannot build"
        .into())
}
