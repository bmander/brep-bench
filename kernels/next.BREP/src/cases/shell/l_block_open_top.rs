//! The L through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15), extruded 8 along +z, shelled
//! inward to a wall 1 thick, open at its top, the wall every point of the plate within 1 of its
//! faces other than the top: at the concave edge x = 5, y = 5 the wall's inside is the quarter
//! cylinder of radius 1 about the edge.
//!
//! The kernel's `offset_shell` joins the moved faces sharp. Its whole-solid offset closes them
//! round: `offset_solid` with `OffsetCorners::Rounded`, moving every face 1 inward, is every point
//! at least 1 inside the body. Of the L run on 2 past its top, that is the cavity, every point of
//! the plate at least 1 from its walls and its bottom, standing out of the top; the plate less it
//! is the wall.
use crate::solids::shell::less;
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, offset_solid, BrepSolid, OffsetCorners};

const THICKNESS: f64 = 1.0;

pub fn build() -> Result<BrepSolid, Refusal> {
    let l = polygon(&[
        p(0.0, 0.0, 0.0), p(20.0, 0.0, 0.0), p(20.0, 5.0, 0.0),
        p(5.0, 5.0, 0.0), p(5.0, 15.0, 0.0), p(0.0, 15.0, 0.0),
    ])?;
    let plate = extrude_profile_brep(&l, p(0.0, 0.0, 1.0), 8.0)?;
    let run_on = extrude_profile_brep(&l, p(0.0, 0.0, 1.0), 8.0 + 2.0 * THICKNESS)?;
    let cavity = offset_solid(&run_on, -THICKNESS, OffsetCorners::Rounded)?.solid;
    less(&plate, &cavity)
}
