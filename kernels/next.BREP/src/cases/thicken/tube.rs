//! The sheet of the whole cylinder of radius 5 about the z axis from z = 0 to z = 10, one face
//! closed round on itself, thickened by 1 away from the axis: the tube between radii 5 and 6.
use crate::solids::offset::thicken;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cylinder_surface, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let sheet = make_cylinder_surface(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 10.0)?;
    thicken(&sheet, &[], 1.0, |q| p(q.x, q.y, 0.0))
}
