//! The sheet of five faces of the cube from the origin to (10, 10, 10), its floor and four walls
//! with no top, thickened by 1 into the cube, each moved face trimmed where it meets the others:
//! the cube less the box from (1, 1, 1) to (9, 9, 10).
//!
//! next.BREP thickens one face at a time, so the tray is thickened as the faces of the cube by
//! the kernel's shell, open at the top.
use crate::solids::offset::{shell, tray};
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    let (cube, opening) = tray()?;
    shell(&cube, &opening, 1.0)
}
