//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, shelled inward to a wall 1 thick,
//! open at its curved side: only the ends are walls, and the result is two separate discs, from
//! z = 0 to z = 1 and from z = 9 to z = 10.
use crate::solids::p;
use crate::solids::shell::shell;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cylinder = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 10.0)?;
    let side = p(3.0, 4.0, 5.0);
    shell(&cylinder, &[side], 1.0)
}
