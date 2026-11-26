//! The plate from (−100, −100, 0) to (100, 100, 40), with a hole of radius 30 drilled through it
//! along the z axis, grown by 30, exactly the hole's radius: the solid of every point within 30 of
//! it (`offset/drilled-plate-grown-by-its-hole` ten times larger). The plate grows as a box does;
//! the hole fills, and over each end of it the grown face dips in a dimple, the part inside the
//! rim of a horn torus, down to a cusp on the axis.
//!
//! The rectangle extruded, the hole a circle extruded through it and cut; then cadrs's offset
//! with rounded edges.
use crate::bench::{block, cut, cylinder, p3, v3};
use crate::shared::offset::grown;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let plate = block(k, p3(-100.0, -100.0, 0.0), p3(100.0, 100.0, 40.0))?;
    let drill = cylinder(k, p3(0.0, 0.0, -10.0), v3(0.0, 0.0, 1.0), 30.0, 60.0)?;
    let drilled = cut(k, plate, drill)?;
    grown(k, drilled, 30.0)
}
