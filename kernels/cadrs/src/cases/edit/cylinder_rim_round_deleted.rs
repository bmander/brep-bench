//! The cylinder of radius 6 on the z axis from z = 0 to z = 10, its top rim rounded to radius 2;
//! then the round, a quarter torus, deleted, the top and the side run on to meet in the rim again:
//! the cylinder.
//!
//! The cylinder is a circle extruded and its rim filleted, radius 2; then Delete face takes the
//! round.
use crate::bench::{cylinder, p3, v3};
use crate::shared::edit::deleted;
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let c = cylinder(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 6.0, 10.0)?;
    let rim = k.edge_at(c, p3(0.0, -6.0, 10.0))?;
    let rounded = fillet(k, c, &[rim], 2.0)?;
    // the round at 45° round its tube, whose center line is the circle of radius 4 at z = 8
    let s = 2f64.sqrt();
    let round = k.faces_at(rounded, &[p3(0.0, -(4.0 + s), 8.0 + s)])?;
    deleted(k, rounded, &round)
}
