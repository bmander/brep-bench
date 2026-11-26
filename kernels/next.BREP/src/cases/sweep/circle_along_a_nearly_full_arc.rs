//! A circle of radius 2 swept along 350° of the arc of radius 10 about the z axis, from (10, 0, 0)
//! anticlockwise, the section square to the path: the kernel's path sweep, as
//! `sweep/circle-along-arc` sweeps its quarter turn.
use crate::solids::p;
use crate::solids::sweep::circle;
use crate::Refusal;
use brep_kernel::{make_arc, sweep_profile_along_path, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (x, y, z) = (p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0), p(0.0, 0.0, 1.0));
    let path = make_arc(p(0.0, 0.0, 0.0), x, y, 10.0, 0.0, 350f64.to_radians())?;
    // the section at the start, in the xz plane square to the arc
    let section = circle(p(10.0, 0.0, 0.0), x, z, 2.0)?;
    Ok(sweep_profile_along_path(&section, &path, None)?)
}
