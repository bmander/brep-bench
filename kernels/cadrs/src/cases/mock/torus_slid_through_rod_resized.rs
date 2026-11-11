//! `swept/torus-slid-through-rod` at other numbers: the wheel centred (11, 0, −9), tube 1.8.
use crate::bench::{p3, torus, v3};
use crate::shared::swept::{cut_swept, rod, slide};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};
pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let body = rod(k)?;
    let wheel = torus(k, p3(11.0, 0.0, -9.0), v3(0.0, 1.0, 0.0), 6.0, 1.8)?;
    cut_swept(k, body, wheel, &slide(v3(0.0, 0.0, 38.0)))
}
