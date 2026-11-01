//! The plate from (−10, −10, 0) to (10, 10, 4) with a hole of radius 3 drilled through it along
//! the z axis, grown by 1 to its parallel body: the plate grown as a box is, the hole narrowed to
//! radius 2 and flaring out at each rim through the inner quarter of a torus's tube.
//!
//! The hole is cut by a cylinder running past both faces of the plate.
use crate::solids::offset::{parallel, subtract};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(-10.0, -10.0, 0.0), 20.0, 20.0, 4.0)?;
    let drill = make_cylinder_brep(p(0.0, 0.0, -1.0), p(0.0, 0.0, 1.0), 3.0, 6.0)?;
    let drilled = subtract(&plate, &drill)?;
    Ok(parallel(&drilled, 1.0)?)
}
