//! `singular/sectors-sharing-an-apex` ten times larger, built as it is at every length: the two
//! sectors of the ball of radius 100, within 45° of +z and of that turned 60° about y, and their
//! common part.
use super::sectors_sharing_an_apex::shared_apex;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    shared_apex(100.0)
}
