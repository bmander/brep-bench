//! A wheel, a disc of radius 40 about the vertical line through (−50, 0) from z = −8 to z = −2,
//! cut by a plus-shaped tool (arms 30 long and 8 wide, from z = −10 to 0) turning about the z axis
//! in mesh with it, the wheel turning −2/3 as fast about its own axis, over one turn of the wheel.
//!
//! Refused: arris has no sweep of a solid. Seen from the wheel the tool rolls, turning about its
//! own axis while that axis goes round the wheel's, and the region it sweeps there is bounded by
//! the envelopes of the arms' sides and the trochoids their corners trace; arris has no such
//! curves, and computing them to build the cut would be making the envelope outside the kernel.
use crate::Refusal;
use arris::topo::{Body, Model};

pub fn build(_: &mut Model) -> Result<Body, Refusal> {
    Err("arris has no sweep of a solid along a motion; the wheel's cut is bounded by envelopes of \
         the plus's sides and trochoids of its corners, which it cannot build"
        .into())
}
