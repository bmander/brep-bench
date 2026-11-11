//! `swept/ball-swung-groove` at other numbers: a ball of radius 2.5 from (10, 0, 0), swung 70°.
use crate::bench::{ball, block, p3};
use crate::shared::swept::{cut_swept, swing, z_axis};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};
pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let body = block(k, p3(-6.0, -6.0, -8.0), p3(18.0, 18.0, 0.0))?;
    let tool = ball(k, p3(10.0, 0.0, 0.0), 2.5)?;
    cut_swept(k, body, tool, &swing(z_axis(), 0.0, 70.0))
}
