//! A gear blank, a disc of radius 22 about the y axis from y = 0 to y = 6, with one tooth space
//! cut by a rack tooth (module 2, 20° pressure angle) whose pitch line rolls without slipping on
//! the blank's pitch circle of radius 20, 60° each side of the middle of the tooth.
//!
//! Refused: arris has no sweep of a solid, and none along a rolling motion. The space's flanks
//! are involutes of the base circle, the envelopes of the rack's flanks as it rolls, and its root
//! the trochoid the tooth's tip corners trace; arris has no involute or trochoid curve, and
//! building either from its formula as a spline would be computing the envelope outside the
//! kernel.
use crate::Refusal;
use arris::topo::{Body, Model};

pub fn build(_: &mut Model) -> Result<Body, Refusal> {
    Err("arris has no sweep of a solid along a rolling motion, and no involute or trochoid: the \
         tooth space's flanks and root are envelopes of the rolling rack it cannot build"
        .into())
}
