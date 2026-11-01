//! The sheet of the side of the cone frustum about the z axis, radius 6 at z = 0 to 3 at z = 4,
//! thickened by 1 along its normal away from the axis: its section a rectangle 5 by 1, leaning.
use crate::solids::offset::thicken;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cone_surface, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let sheet = make_cone_surface(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 6.0, 3.0, 4.0)?;
    thicken(&sheet, &[], 1.0, |q| p(q.x, q.y, 0.0))
}
