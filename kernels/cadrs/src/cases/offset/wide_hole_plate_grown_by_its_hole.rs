//! The plate from (−30, −30, 0) to (30, 30, 4), with a hole of radius 20 drilled through it along
//! the z axis, grown by 20, exactly the hole's radius: the solid of every point within 20 of it.
//! The plate grows as a box does; the hole fills, and over each end of it the grown face dips in a
//! dimple, the part inside the rim of a horn torus, down to a cusp on the axis.
//!
//! The rectangle extruded, the hole a circle extruded through it and cut; then cadrs's offset
//! with rounded edges.
use crate::bench::{block, cut, cylinder, p3, v3};
use crate::shared::offset::grown;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let plate = block(k, p3(-30.0, -30.0, 0.0), p3(30.0, 30.0, 4.0))?;
    let drill = cylinder(k, p3(0.0, 0.0, -1.0), v3(0.0, 0.0, 1.0), 20.0, 6.0)?;
    let drilled = cut(k, plate, drill)?;
    grown(k, drilled, 20.0)
}
