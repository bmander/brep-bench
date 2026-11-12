//! The cylinder of radius 8 on the z axis from z = −4 to z = 0 with the cylinder of radius 3 from z
//! = 0 to z = 8 standing on it, the concave edge at its foot rounded to radius 1; then the round, a
//! quarter torus, deleted, the plate's top and the boss's side run on to meet in that edge: the
//! plate and the boss, sharp where they meet.
//!
//! The two cylinders are united and the edge at the boss's foot filleted, radius 1; then Delete
//! face takes the round.
use crate::bench::{cylinder, p3, union, v3};
use crate::shared::edit::deleted;
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let z = v3(0.0, 0.0, 1.0);
    let plate = cylinder(k, p3(0.0, 0.0, -4.0), z, 8.0, 4.0)?;
    let boss = cylinder(k, p3(0.0, 0.0, 0.0), z, 3.0, 8.0)?;
    let b = union(k, plate, boss)?;
    let foot = k.edge_at(b, p3(0.0, -3.0, 0.0))?;
    let rounded = fillet(k, b, &[foot], 1.0)?;
    // the round at 45° round its tube, whose center line is the circle of radius 4 at z = 1
    let s = 0.5f64.sqrt();
    let round = k.faces_at(rounded, &[p3(0.0, -(4.0 - s), 1.0 - s)])?;
    deleted(k, rounded, &round)
}
