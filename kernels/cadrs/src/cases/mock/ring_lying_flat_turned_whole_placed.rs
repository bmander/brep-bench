//! `swept/ring-lying-flat-turned-whole`, placed.
use crate::bench::{p3, torus, v3};
use crate::shared::mock::{axis, place};
use crate::shared::swept::{swung, z_axis};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};
pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let ring = torus(k, p3(8.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 3.0, 1.0)?;
    let ring = place(k, ring)?;
    swung(k, ring, axis(z_axis()), 0.0, 360.0)
}
