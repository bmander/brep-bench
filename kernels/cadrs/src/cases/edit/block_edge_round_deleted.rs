//! The box from the origin to (20, 10, 10), the edge where its top meets its side x = 20 rounded to
//! radius 2; then the round deleted, the top and the side run on to meet in that edge again: the
//! box.
//!
//! The block is a rectangle extruded and the edge filleted, radius 2; then Delete face takes the
//! round.
use crate::bench::{block, p3};
use crate::shared::edit::deleted;
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = block(k, p3(0.0, 0.0, 0.0), p3(20.0, 10.0, 10.0))?;
    let edge = k.edge_at(b, p3(20.0, 5.0, 10.0))?;
    let rounded = fillet(k, b, &[edge], 2.0)?;
    // the round at its middle: 2 from its axis, the line x = 18, z = 8, at 45°
    let s = 2f64.sqrt();
    let round = k.faces_at(rounded, &[p3(18.0 + s, 5.0, 8.0 + s)])?;
    deleted(k, rounded, &round)
}
