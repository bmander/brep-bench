//! `swept/ball-swung-groove`, placed.
use crate::bench::{ball, block, p3};
use crate::shared::mock::{axis, place};
use crate::shared::swept::{cut_swept, swing, z_axis};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};
pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let body = block(k, p3(-6.0, -6.0, -8.0), p3(18.0, 18.0, 0.0))?;
    let body = place(k, body)?;
    let tool = ball(k, p3(12.0, 0.0, 0.0), 3.0)?;
    let tool = place(k, tool)?;
    cut_swept(k, body, tool, &swing(axis(z_axis()), 0.0, 90.0))
}
