//! A dumbbell-shaped plate, the outline through (0, 0), (10, 0), (10, 4), (14, 4), (14, 0),
//! (24, 0), (24, 10), (14, 10), (14, 6), (10, 6), (10, 10), (0, 10) extruded 6 along +z, shelled
//! inward to a wall 1.5 thick, open at its top, the wall every point of the plate within 1.5 of its
//! faces other than the top: the neck, 2 wide, fills, and each pocket bulges towards it in arcs of
//! radius 1.5 about the neck's corners.
//!
//! The kernel's `offset_shell` joins the moved faces sharp. Its whole-solid offset closes them
//! round: `offset_solid` with `OffsetCorners::Rounded`, moving every face 1.5 inward, is every
//! point at least 1.5 inside the body. Of the plate run on 3 past its top, that is the cavity,
//! every point of the plate at least 1.5 from its walls and its bottom, standing out of the top;
//! the plate less it is the wall.
use crate::solids::shell::less;
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, offset_solid, BrepSolid, OffsetCorners};

const THICKNESS: f64 = 1.5;

pub fn build() -> Result<BrepSolid, Refusal> {
    let outline = polygon(&[
        p(0.0, 0.0, 0.0), p(10.0, 0.0, 0.0), p(10.0, 4.0, 0.0), p(14.0, 4.0, 0.0),
        p(14.0, 0.0, 0.0), p(24.0, 0.0, 0.0), p(24.0, 10.0, 0.0), p(14.0, 10.0, 0.0),
        p(14.0, 6.0, 0.0), p(10.0, 6.0, 0.0), p(10.0, 10.0, 0.0), p(0.0, 10.0, 0.0),
    ])?;
    let plate = extrude_profile_brep(&outline, p(0.0, 0.0, 1.0), 6.0)?;
    let run_on = extrude_profile_brep(&outline, p(0.0, 0.0, 1.0), 6.0 + 2.0 * THICKNESS)?;
    let cavity = offset_solid(&run_on, -THICKNESS, OffsetCorners::Rounded)?.solid;
    less(&plate, &cavity)
}
