//! The box from the origin to (10, 8, 6) shelled outward to a wall 1 thick, open at its top, the
//! wall every point outside the box within 1 of its other faces: the box grown by 1 and cut off at
//! z = 6, less the box. Its outside is rounded, quarter cylinders about the edges and eighths of a
//! ball at the bottom corners.
//!
//! The kernel's `offset_shell` joins the moved faces sharp. Its whole-solid offset closes them
//! round: `offset_solid` with `OffsetCorners::Rounded` is the parallel body, every point within 1
//! of the box. That cut by the plane of the top, less the box, is the wall.
use crate::solids::p;
use crate::solids::shell::less;
use crate::Refusal;
use brep_kernel::{make_box_brep, offset_solid, split_solid_by_plane, BrepSolid, OffsetCorners};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 8.0, 6.0)?;
    let grown = offset_solid(&block, 1.0, OffsetCorners::Rounded)?.solid;
    let (below, _) = split_solid_by_plane(&grown, p(0.0, 0.0, 6.0), p(0.0, 0.0, 1.0))?;
    less(&below, &block)
}
