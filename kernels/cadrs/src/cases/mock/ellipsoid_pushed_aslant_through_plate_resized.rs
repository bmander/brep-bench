//! `swept/ellipsoid-pushed-aslant-through-plate` at other numbers: semi-axes (2.5, 2.2, 1.2),
//! centred (1.5, −2.5, −4), pushed along (−3, 5, 12).
use crate::bench::{ball, block, cut, moved, p3, v3};
use crate::shared::swept::slid;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Kernel};
pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let plate = block(k, p3(-15.0, -15.0, 0.0), p3(15.0, 15.0, 4.0))?;
    let unit = ball(k, p3(0.0, 0.0, 0.0), 1.0)?;
    let stretched = k.scale_axes(unit, p3(0.0, 0.0, 0.0), v3(2.5, 2.2, 1.2))?.first()?;
    let e = moved(k, stretched, v3(1.5, -2.5, -4.0))?;
    let swept = slid(k, e, v3(-3.0, 5.0, 12.0))?;
    cut(k, plate, swept)
}
