//! A cube of side 10 from the origin to (10, 10, 10), united with a cylinder of radius 5 and
//! height 10 standing on the xy plane, its axis at x = 10, y = 5: half of it inside the cube, its
//! wall tangent to the faces y = 0 and y = 10 along the cube's edges at x = 10.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cube = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 10.0, 10.0)?;
    let boss = make_cylinder_brep(p(10.0, 5.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 10.0)?;
    Ok(union(&cube, &boss)?)
}
