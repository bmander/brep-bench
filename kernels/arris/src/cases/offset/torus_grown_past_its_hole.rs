//! A solid torus about the z axis, its tube of radius 2 about a middle circle of radius 4,
//! grown by 3, so that it closes over its hole.
//!
//! Refused: arris's `offset_faces` moves the torus's one face out 3, to a torus whose tube (radius
//! 5) is wider than its middle circle (radius 4), so that it would pass through the axis, and
//! refuses it ("the offset drives the face's surface through zero"); it does not trim an offset
//! that crosses itself.
use crate::Refusal;
use arris::topo::{Body, Model};

pub fn build(_: &mut Model) -> Result<Body, Refusal> {
    Err("arris has no offset that trims an offset that crosses itself: `offset_faces` refuses the torus's face moved past the axis (\"the offset drives the face's surface through zero\")".into())
}
