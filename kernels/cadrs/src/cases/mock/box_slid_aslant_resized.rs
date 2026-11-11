//! `swept/box-slid-aslant` at other numbers: the box (0, 0, 0)–(7, 9, 3) slid along (−5, 4, 6).
use crate::bench::{block, p3, v3};
use crate::shared::swept::slid;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};
pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let start = block(k, p3(0.0, 0.0, 0.0), p3(7.0, 9.0, 3.0))?;
    slid(k, start, v3(-5.0, 4.0, 6.0))
}
