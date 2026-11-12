//! The box from the origin to (20, 10, 10), drilled through along z by a hole of radius 2 through
//! (10, 5) and countersunk at the top by the cone from radius 2 at z = 8 to radius 4 at z = 10;
//! then the countersink deleted, the hole's wall and the top run on to meet: the block with the
//! plain hole.
//!
//! The block is drilled and countersunk by Boolean cuts; then Delete face takes the countersink.
use crate::bench::{block, cone, cut, cylinder, p3, v3};
use crate::shared::edit::deleted;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let z = v3(0.0, 0.0, 1.0);
    let b = block(k, p3(0.0, 0.0, 0.0), p3(20.0, 10.0, 10.0))?;
    let drill = cylinder(k, p3(10.0, 5.0, -1.0), z, 2.0, 12.0)?;
    let drilled = cut(k, b, drill)?;
    // the countersink's cone, run on past the top to z = 11, where its radius is 5
    let sink = cone(k, p3(10.0, 5.0, 8.0), z, 2.0, 5.0, 3.0)?;
    let countersunk = cut(k, drilled, sink)?;
    let countersink = k.faces_at(countersunk, &[p3(13.0, 5.0, 9.0)])?;
    deleted(k, countersunk, &countersink)
}
