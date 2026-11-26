//! The solid a ball (radius 20, its centre starting at (15, 0, 0)) fills as it is screwed about the
//! z axis, rising 30 a turn, for two whole turns: everything within 20 of its centre's helix
//! (`swept/ball-screwed-round-a-tight-helix` ten times larger).
//!
//! cadrs's sweep of a solid (a turn and a slide along its axis at once: a screw).
use crate::bench::{ball, p3};
use crate::shared::swept::{screwed, z_axis};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = ball(k, p3(15.0, 0.0, 0.0), 20.0)?;
    screwed(k, b, z_axis(), 30.0, 0.0, 720.0)
}
