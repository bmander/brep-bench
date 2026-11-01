//! The sheet of the rectangle in the xy plane from (0, 0, 0) to (10, 8, 0), thickened by 2 to its
//! +z side: the box from the origin to (10, 8, 2).
use crate::solids::offset::thicken;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_plane, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let sheet = make_plane(p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0), 10.0, 8.0)?;
    thicken(&sheet, &[], 2.0, |_| p(0.0, 0.0, 1.0))
}
