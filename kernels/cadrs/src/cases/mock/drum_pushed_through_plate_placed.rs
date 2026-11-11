//! `swept/drum-pushed-through-plate`, placed.
use crate::bench::{block, cut, cylinder, p3, v3};
use crate::shared::mock::{place, vector};
use crate::shared::swept::slid;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};
pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let plate = block(k, p3(0.0, -5.0, 0.0), p3(20.0, 5.0, 4.0))?;
    let plate = place(k, plate)?;
    let drum = cylinder(k, p3(6.0, 0.0, -5.0), v3(1.0, 0.0, 0.0), 2.0, 8.0)?;
    let drum = place(k, drum)?;
    let swept = slid(k, drum, vector(v3(0.0, 0.0, 16.0)))?;
    cut(k, plate, swept)
}
