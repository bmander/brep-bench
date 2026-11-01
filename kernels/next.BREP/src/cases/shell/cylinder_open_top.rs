//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, shelled inward to a wall 1 thick,
//! open at its top: a cup, the cylinder less the cylinder of radius 4 from z = 1 to z = 10.
use crate::solids::p;
use crate::solids::shell::shell;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cylinder = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 10.0)?;
    let top = p(1.0, 1.0, 10.0);
    shell(&cylinder, &[top], 1.0)
}
