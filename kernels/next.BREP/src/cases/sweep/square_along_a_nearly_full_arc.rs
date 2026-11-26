//! A square of side 2 swept along 350° of the arc of radius 10 about the z axis, from (10, 0, 0)
//! anticlockwise, the section square to the path: the kernel's path sweep.
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{make_arc, sweep_profile_along_path, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (x, y) = (p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0));
    let path = make_arc(p(0.0, 0.0, 0.0), x, y, 10.0, 0.0, 350f64.to_radians())?;
    // the section at the start, in the xz plane square to the arc
    let square = polygon(&[p(9.0, 0.0, -1.0), p(11.0, 0.0, -1.0), p(11.0, 0.0, 1.0),
        p(9.0, 0.0, 1.0)])?;
    Ok(sweep_profile_along_path(&square, &path, None)?)
}
