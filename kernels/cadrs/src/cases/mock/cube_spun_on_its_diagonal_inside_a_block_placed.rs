//! `swept/cube-spun-on-its-diagonal-inside-a-block`, placed.
use crate::bench::{block, cut, p3, v3};
use crate::shared::mock::{axis, place};
use crate::shared::swept::swung;
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, DefaultKernel};
pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = block(k, p3(-3.0, -3.0, -3.0), p3(3.0, 3.0, 3.0))?;
    let b = place(k, b)?;
    let cube = block(k, p3(-1.0, -1.0, -1.0), p3(1.0, 1.0, 1.0))?;
    let cube = place(k, cube)?;
    let swept = swung(k, cube, axis(Axis::new(p3(-1.0, -1.0, -1.0), v3(1.0, 1.0, 1.0))), 0.0, 360.0)?;
    cut(k, b, swept)
}
