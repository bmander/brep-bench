//! `swept/torus-slid-through-rod`, placed.
use crate::bench::{p3, torus, v3};
use crate::shared::mock::{place, vector};
use crate::shared::swept::{cut_swept, rod, slide};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};
pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let body = rod(k)?;
    let body = place(k, body)?;
    let wheel = torus(k, p3(11.5, 0.0, -9.0), v3(0.0, 1.0, 0.0), 6.0, 2.0)?;
    let wheel = place(k, wheel)?;
    cut_swept(k, body, wheel, &slide(vector(v3(0.0, 0.0, 38.0))))
}
