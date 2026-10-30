//! A gear blank, a disc of radius 22 about the y axis from y = 0 to y = 6, with one tooth space
//! cut by a rack tooth (module 2, 20° pressure angle) whose pitch line rolls without slipping on
//! the blank's pitch circle of radius 20, 60° each side of the middle of the tooth.
//!
//! Refused: vcad has no sweep of a solid (`Solid::sweep`, `sweep_cylindrical` and `loft` move a
//! sketch profile), and none along a rolling motion. The space's flanks are involutes of the base
//! circle, the envelopes of the rack's flanks as it rolls, and its root the trochoid the tooth's
//! tip corners trace; a vcad sketch holds only lines and circular arcs, and building either curve
//! from its formula would be computing the envelope outside the kernel.
use vcad_kernel::Solid;

pub fn build() -> Result<Solid, crate::Refusal> {
    Err("vcad has no sweep of a solid along a rolling motion, and no involute or trochoid (a \
         sketch holds only lines and circular arcs): the tooth space's flanks are envelopes of \
         the rack's"
        .into())
}
