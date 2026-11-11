//! `swept/ellipsoid-pushed-aslant-through-plate`, placed.
use crate::bench::{ball, block, cut, moved, p3, v3};
use crate::shared::mock::{place, vector};
use crate::shared::swept::slid;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Kernel};
pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let plate = block(k, p3(-15.0, -15.0, 0.0), p3(15.0, 15.0, 4.0))?;
    let plate = place(k, plate)?;
    let unit = ball(k, p3(0.0, 0.0, 0.0), 1.0)?;
    let stretched = k.scale_axes(unit, p3(0.0, 0.0, 0.0), v3(3.0, 2.0, 1.5))?.first()?;
    let e = moved(k, stretched, v3(-2.0, -1.5, -4.0))?;
    let e = place(k, e)?;
    let swept = slid(k, e, vector(v3(4.0, 3.0, 12.0)))?;
    cut(k, plate, swept)
}
