//! The box from the origin to (30, 20, 10), its four vertical edges rounded to radius 4. All
//! eight sides (four planes, four quarter cylinders) drafted 5°, pull +z, about the neutral plane
//! z = 0, each leaning in as it rises: the planes turn about their bottom edges and the quarter
//! cylinders become quarter cones about the same axes, staying tangent to them. The top is
//! trimmed to the drafted sides.
//!
//! The rectangle extruded, its vertical edges filleted, then cadrs's Draft of every side.
use crate::bench::{block, p3, v3};
use crate::shared::draft::{drafted, is_side};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Kernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = block(k, p3(0.0, 0.0, 0.0), p3(30.0, 20.0, 10.0))?;
    let verticals = k.edges_at(b,
        &[p3(0.0, 0.0, 5.0), p3(30.0, 0.0, 5.0), p3(30.0, 20.0, 5.0), p3(0.0, 20.0, 5.0)],
    )?;
    let rounded = k.fillet(b, &verticals, 4.0)?.bodies[0];
    let sides = k.faces_where(rounded, &is_side)?;
    drafted(k, rounded, sides, 5.0, v3(0.0, 0.0, 1.0), p3(0.0, 0.0, 0.0))
}
