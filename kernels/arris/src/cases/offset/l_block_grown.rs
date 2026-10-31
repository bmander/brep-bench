//! The L-shaped plate through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15), extruded 8
//! along +z, grown by 1: its convex edges rounded, its concave edge kept sharp.
//!
//! Refused: arris's `offset_faces` moves every face of a solid along its normal and joins the moved
//! faces where they meet, so at a convex edge of an outward offset the join is sharp. Growing by
//! every point within 1 rounds each convex edge and corner, and arris has no rounded join.
use crate::Refusal;
use arris::topo::{Body, Model};

pub fn build(_: &mut Model) -> Result<Body, Refusal> {
    Err("arris has no offset with rounded joins: `offset_faces` moves every face and joins the moved faces sharp, where they meet".into())
}
