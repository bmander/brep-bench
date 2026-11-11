//! The box from the origin to (30, 10, 10), its four sides drafted 30°, pull +z, about the
//! neutral plane z = 0. The long sides meet at height 5√3, below the top, so the draft consumes
//! the top face: a hip roof of five faces, its ridge from (5, 5, 5√3) to (25, 5, 5√3).
//!
//! The rectangle extruded, then cadrs's Draft of its four sides.
use crate::bench::{block, p3, v3};
use crate::shared::draft::drafted;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = block(k, p3(0.0, 0.0, 0.0), p3(30.0, 10.0, 10.0))?;
    let sides = k.faces_at(b,
        &[p3(0.0, 5.0, 5.0), p3(30.0, 5.0, 5.0), p3(15.0, 0.0, 5.0), p3(15.0, 10.0, 5.0)],
    )?;
    drafted(k, b, sides, 30.0, v3(0.0, 0.0, 1.0), p3(0.0, 0.0, 0.0))
}
