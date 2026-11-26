//! `offset/drilled-plate-grown-by-its-hole` ten times larger: the plate from (−100, −100, 0) to
//! (100, 100, 40) with a hole of radius 30 drilled through it along the z axis, grown by 30,
//! exactly the hole's radius, to its parallel body: the hole fills, and each end of it leaves a
//! dimple in the grown face, the inside of a horn torus. Built as the original is, every length
//! times 10.
//!
//! The hole is cut by a cylinder running past both faces of the plate.
use crate::solids::offset::{parallel, subtract};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(-100.0, -100.0, 0.0), 200.0, 200.0, 40.0)?;
    let drill = make_cylinder_brep(p(0.0, 0.0, -10.0), p(0.0, 0.0, 1.0), 30.0, 60.0)?;
    let drilled = subtract(&plate, &drill)?;
    Ok(parallel(&drilled, 30.0)?)
}
