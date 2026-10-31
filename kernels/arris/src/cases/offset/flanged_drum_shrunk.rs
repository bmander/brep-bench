//! A drum of radius 8 from z = 0 to z = 6, with a flange 1.6 thick out to radius 11 round its
//! side from z = 2.2 to z = 3.8, shrunk by 1, so that the flange is gone.
//!
//! Refused: arris's `offset_faces` keeps the solid's topology: the flange's faces, moved 1 towards
//! each other, cross, and it refuses them ("the offset would make the edge or face vanish or turn
//! inside out") and does not trim them away. Where the flange meets the drum, a concave edge
//! shrunk, it has no rounded join either.
use crate::Refusal;
use arris::topo::{Body, Model};

pub fn build(_: &mut Model) -> Result<Body, Refusal> {
    Err("arris has no offset that trims an offset that crosses itself, nor rounded joins: `offset_faces` keeps the topology, and refuses the flange's faces moved through each other (\"the offset would make the edge or face vanish or turn inside out\")".into())
}
