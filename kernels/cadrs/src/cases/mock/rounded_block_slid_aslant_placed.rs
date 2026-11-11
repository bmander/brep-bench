//! `swept/rounded-block-slid-aslant`, placed.
use crate::bench::{block, p3, v3};
use crate::shared::fillet::fillet;
use crate::shared::mock::{place, vector};
use crate::shared::swept::slid;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};
pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = block(k, p3(0.0, 0.0, 0.0), p3(10.0, 6.0, 4.0))?;
    let edges = k.edges_but_seams(b)?;
    let rounded = fillet(k, b, &edges, 1.0)?;
    let rounded = place(k, rounded)?;
    slid(k, rounded, vector(v3(8.0, 5.0, 3.0)))
}
