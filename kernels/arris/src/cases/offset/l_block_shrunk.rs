//! The L-shaped plate through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15), extruded 8
//! along +z, shrunk by 1: its concave edge rounded, its convex edges kept sharp.
//!
//! Refused: arris's `offset_faces` moves every face of a solid along its normal and joins the moved
//! faces where they meet, so the inside corner's edge, concave, would stay sharp. Shrinking by
//! every point at least 1 inside rounds it, and arris has no rounded join.
use crate::Refusal;
use arris::topo::{Body, Model};

pub fn build(_: &mut Model) -> Result<Body, Refusal> {
    Err("arris has no offset with rounded joins: `offset_faces` moves every face and joins the moved faces sharp, so the concave edge would not be rounded".into())
}
