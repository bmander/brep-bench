//! `swept/ring-lying-flat-turned-whole` at other numbers: the torus at (6.5, 0, 0), radii 2.2, 0.9.
use crate::bench::{p3, torus, v3};
use crate::shared::swept::{swung, z_axis};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};
pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let ring = torus(k, p3(6.5, 0.0, 0.0), v3(0.0, 0.0, 1.0), 2.2, 0.9)?;
    swung(k, ring, z_axis(), 0.0, 360.0)
}
