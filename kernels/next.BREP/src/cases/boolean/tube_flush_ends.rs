//! A cylinder of radius 5 on the z axis from z = 0 to 10, less a cylinder of radius 3 on the same
//! axis running exactly from z = 0 to 10, its ends in the first's.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let outer = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 10.0)?;
    let inner = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 3.0, 10.0)?;
    Ok(cut(&outer, &inner)?)
}
