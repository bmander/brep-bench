//! `swept/ring-slid-flat-past-its-hole`, placed.
use crate::bench::{p3, torus, v3};
use crate::shared::mock::{place, vector};
use crate::shared::swept::slid;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};
pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let ring = torus(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 6.0, 2.0)?;
    let ring = place(k, ring)?;
    slid(k, ring, vector(v3(15.0, 0.0, 0.0)))
}
