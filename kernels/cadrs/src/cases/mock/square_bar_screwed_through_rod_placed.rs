//! `swept/square-bar-screwed-through-rod`, placed.
use crate::bench::{block, cut, p3};
use crate::shared::mock::{axis, place};
use crate::shared::swept::{near_axis, rod, screwed, z_axis};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};
pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let bar = block(k, p3(4.0, -8.0, -1.0), p3(12.0, 8.0, 1.0))?;
    let bar = near_axis(k, bar, 5.5)?;
    let bar = place(k, bar)?;
    let swept = screwed(k, bar, axis(z_axis()), 6.0, -90.0, 1350.0)?;
    let r = rod(k)?;
    let r = place(k, r)?;
    cut(k, r, swept)
}
