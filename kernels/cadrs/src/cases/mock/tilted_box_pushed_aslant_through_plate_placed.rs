//! `swept/tilted-box-pushed-aslant-through-plate`, placed.
use crate::bench::{block, cut, p3, v3};
use crate::shared::mock::{place, vector};
use crate::shared::swept::slid;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Kernel};
pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let plate = block(k, p3(-15.0, -15.0, 0.0), p3(15.0, 15.0, 4.0))?;
    let plate = place(k, plate)?;
    let local = block(k, p3(-2.0, -1.5, -1.0), p3(2.0, 1.5, 1.0))?;
    let m = nalgebra::Matrix3::from_columns(&[
        v3(2.0, 1.0, 2.0) / 3.0,
        v3(-2.0, 2.0, 1.0) / 3.0,
        v3(1.0, 2.0, -2.0) / 3.0,
    ]);
    let tilted = k.transform_affine(local, m, v3(-2.0, -1.0, -8.0))?.first()?;
    let tilted = place(k, tilted)?;
    let swept = slid(k, tilted, vector(v3(4.0, 2.0, 18.0)))?;
    cut(k, plate, swept)
}
