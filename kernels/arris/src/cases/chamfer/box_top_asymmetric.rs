//! A box from the origin to (20, 10, 6), the four edges round its top chamfered 1 across the top
//! and 2 down the sides.
//!
//! Refused: arris's `chamfer` cuts an edge at one distance, the same on both of its faces, and
//! arris has no chamfer with a different distance on each face.
use crate::Refusal;
use arris::topo::{Body, Model};

pub fn build(_: &mut Model) -> Result<Body, Refusal> {
    Err("arris has no chamfer with two distances: `chamfer` cuts at one distance on both faces"
        .into())
}
