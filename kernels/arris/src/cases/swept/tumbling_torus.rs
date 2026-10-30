//! The solid a torus (centred at (3, 0, 2), axis along z, radii 1 and 0.5) fills as it turns by
//! t/4 about the vertical line through (2, 0, 0) while seen from a frame turning by t about the
//! line through (0, 0.5, 0.5) along x, for t from −75° to 75°.
//!
//! Refused: arris has no sweep of a solid. The motion keeps no line of the torus fixed, so the
//! sweep is neither a revolution nor a slide of it, and its boundary is the envelope of the
//! torus's places, the curves on it where the motion's velocity is tangent to it; arris has no
//! operation that builds that surface, and it would have to be computed outside the kernel.
use crate::Refusal;
use arris::topo::{Body, Model};

pub fn build(_: &mut Model) -> Result<Body, Refusal> {
    Err("arris has no sweep of a solid along a motion; the tumbling torus's boundary is its \
         envelope, which it cannot build"
        .into())
}
