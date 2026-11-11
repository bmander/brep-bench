//! `swept/cube-spun-on-its-diagonal-inside-a-block` at other numbers: the cube of side 1.6.
use crate::bench::{block, cut, p3, v3};
use crate::shared::swept::swung;
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, DefaultKernel};
pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = block(k, p3(-3.0, -3.0, -3.0), p3(3.0, 3.0, 3.0))?;
    let cube = block(k, p3(-0.8, -0.8, -0.8), p3(0.8, 0.8, 0.8))?;
    let swept = swung(k, cube, Axis::new(p3(-0.8, -0.8, -0.8), v3(1.0, 1.0, 1.0)), 0.0, 360.0)?;
    cut(k, b, swept)
}
