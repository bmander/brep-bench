//! The sheet of five faces of the cube from the origin to (10, 10, 10), its floor and four walls
//! with no top, thickened by 1 out of the cube, the moved faces extended until they meet: the box
//! from (−1, −1, −1) to (11, 11, 10) less the cube.
//!
//! next.BREP thickens one face at a time, so the tray is thickened as the faces of the cube by
//! the kernel's shell grown outward, open at the top.
use crate::solids::offset::{shell, tray};
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    let (cube, opening) = tray()?;
    shell(&cube, &opening, -1.0)
}
