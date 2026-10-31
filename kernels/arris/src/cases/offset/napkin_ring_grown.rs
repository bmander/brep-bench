//! A ball of radius 5 centred on the origin, with a hole of radius 3 bored through it along the
//! z axis, grown by 1.
//!
//! Refused: arris's `offset_faces` moves every face of a solid along its normal and joins the moved
//! faces where they meet, so at a convex edge of an outward offset the join is sharp. Growing by
//! every point within 1 rounds each convex edge and corner, and arris has no rounded join.
use crate::Refusal;
use arris::topo::{Body, Model};

pub fn build(_: &mut Model) -> Result<Body, Refusal> {
    Err("arris has no offset with rounded joins: `offset_faces` moves every face and joins the moved faces sharp, where they meet".into())
}
