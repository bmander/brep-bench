//! A drum of radius 10 from z = 0 to z = 10, with a groove 1.6 wide and 3 deep round its side
//! from z = 4.2 to z = 5.8, grown by 1, so that the groove closes.
//!
//! Refused: arris's `offset_faces` joins sharp, so grown it would not round the drum's edges, and
//! it keeps the solid's topology: the groove's walls, moved 1 towards each other, cross, and it
//! refuses them ("the offset would make the edge or face vanish or turn inside out") and does not
//! trim them away.
use crate::Refusal;
use arris::topo::{Body, Model};

pub fn build(_: &mut Model) -> Result<Body, Refusal> {
    Err("arris has no offset with rounded joins, and none that trims an offset that crosses itself: `offset_faces` joins sharp and keeps the topology, and refuses the groove's walls moved through each other (\"the offset would make the edge or face vanish or turn inside out\")".into())
}
