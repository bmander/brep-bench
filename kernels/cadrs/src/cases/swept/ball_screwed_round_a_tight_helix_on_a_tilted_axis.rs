//! `swept/ball-screwed-round-a-tight-helix` turned rigidly so that its axis runs along
//! w = (1, 2, 2)/3, with u = (2, −2, 1)/3 and v = (2, 1, −2)/3: the solid a ball of radius 2,
//! its centre starting at 1.5u = (1, −1, 1/2), fills as it is screwed about the line through the
//! origin along w, rising 3 a turn, for two whole turns.
//!
//! cadrs's sweep of a solid (a turn about the axis and a slide along it at once: a screw).
use crate::bench::{ball, p3, v3};
use crate::shared::swept::screwed;
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let w = v3(1.0, 2.0, 2.0) / 3.0;
    let b = ball(k, p3(1.0, -1.0, 0.5), 2.0)?;
    screwed(k, b, Axis::new(p3(0.0, 0.0, 0.0), w), 3.0, 0.0, 720.0)
}
