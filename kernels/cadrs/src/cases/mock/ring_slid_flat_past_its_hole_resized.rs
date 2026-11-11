//! `swept/ring-slid-flat-past-its-hole` at other numbers: radii 5 and 1.5, slid 13 along x.
use crate::bench::{p3, torus, v3};
use crate::shared::swept::slid;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};
pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let ring = torus(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 5.0, 1.5)?;
    slid(k, ring, v3(13.0, 0.0, 0.0))
}
