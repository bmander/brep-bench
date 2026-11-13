//! The L through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15), extruded 8 along +z, shelled
//! inward to a wall 1 thick, open at its top, the wall every point of the plate within 1 of its
//! faces other than the top: at the concave edge x = 5, y = 5 the wall's inside is the quarter
//! cylinder of radius 1 about the edge.
//!
//! The kernel's shell joined by arcs (`offset_shell_rounded`), inward, open at the top, picked by
//! a point inside it.
use crate::solids::shell::face_at;
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, offset_shell_rounded, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let l = polygon(&[
        p(0.0, 0.0, 0.0), p(20.0, 0.0, 0.0), p(20.0, 5.0, 0.0),
        p(5.0, 5.0, 0.0), p(5.0, 15.0, 0.0), p(0.0, 15.0, 0.0),
    ])?;
    let plate = extrude_profile_brep(&l, p(0.0, 0.0, 1.0), 8.0)?;
    let top = face_at(&plate, p(2.0, 2.0, 8.0))?;
    Ok(offset_shell_rounded(&plate, &[top], 1.0)?)
}
