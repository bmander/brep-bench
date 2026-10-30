//! A wheel, a disc of radius 40 about the vertical line through (−50, 0) from z = −8 to z = −2,
//! cut by a plus-shaped tool (arms 30 long and 8 wide, from z = −10 to 0) turning about the z axis
//! in mesh with it, the wheel turning −2/3 as fast about its own axis, over one turn of the wheel.
//!
//! Refused: vcad has no sweep of a solid (`Solid::sweep`, `sweep_cylindrical` and `loft` move a
//! sketch profile, not a solid along a motion). Seen from the wheel the tool rolls, turning about
//! its own axis while that axis goes round the wheel's, and the region it sweeps there is bounded
//! by the envelopes of the arms' sides and the trochoids their corners trace; vcad has no such
//! curves, and computing them to build the cut would be making the envelope outside the kernel.
use vcad_kernel::Solid;

pub fn build() -> Result<Solid, crate::Refusal> {
    Err("vcad has no sweep of a solid along a motion (its sweeps move a sketch profile along a \
         curve); the wheel's cut is bounded by envelopes of the rolling tool's sides and the \
         trochoids of its corners, which vcad cannot build"
        .into())
}
