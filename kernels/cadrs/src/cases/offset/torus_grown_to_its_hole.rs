//! A solid torus about the z axis, centered on the origin, its tube of radius 2 about a middle
//! circle of radius 4, grown by 2: every point within 4 of the middle circle, the horn torus,
//! which closes its hole at the origin.
//!
//! The torus is cadrs's torus primitive. Then cadrs's offset with rounded edges.
use crate::bench::{p3, torus, v3};
use crate::shared::offset::grown;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let t = torus(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 4.0, 2.0)?;
    grown(k, t, 2.0)
}
