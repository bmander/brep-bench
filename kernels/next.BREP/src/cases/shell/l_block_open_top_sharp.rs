//! The L through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15), extruded 8 along +z, shelled
//! inward to a wall 1 thick, open at its top, every pair of moved walls extended until they meet:
//! at the concave edge x = 5, y = 5 the moved walls x = 4 and y = 4 meet sharp in x = 4, y = 4.
//! The kernel's shell joins moved faces sharp.
use crate::solids::shell::shell;
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let l = polygon(&[
        p(0.0, 0.0, 0.0), p(20.0, 0.0, 0.0), p(20.0, 5.0, 0.0),
        p(5.0, 5.0, 0.0), p(5.0, 15.0, 0.0), p(0.0, 15.0, 0.0),
    ])?;
    let plate = extrude_profile_brep(&l, p(0.0, 0.0, 1.0), 8.0)?;
    let top = p(2.0, 2.0, 8.0);
    shell(&plate, &[top], 1.0)
}
