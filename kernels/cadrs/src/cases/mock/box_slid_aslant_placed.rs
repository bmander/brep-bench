//! `swept/box-slid-aslant`, placed.
use crate::bench::{block, p3, v3};
use crate::shared::mock::{place, vector};
use crate::shared::swept::slid;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};
pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let start = block(k, p3(0.0, 0.0, 0.0), p3(10.0, 6.0, 4.0))?;
    let start = place(k, start)?;
    slid(k, start, vector(v3(8.0, 5.0, 3.0)))
}
