//! A sheet bent as sheet metal is: the profile in the xz plane that runs along the line from (−10,
//! 0, 0) to (0, 0, 0), then round the quarter circle of radius 2 about (0, 0, 2) to (2, 0, 2), then
//! up the line to (2, 0, 12), swept along +y from y = 0 to y = 8. It is three faces, meeting
//! tangentially: a flat floor, a bend that is a quarter of a cylinder of radius 2 about the line
//! through (0, ·, 2) along y, and a flat upright wall.
//!
//! Refused. cad-kernel has no thickening of a sheet: it makes sheets (`build::sheet`,
//! `build::sheets`) and cuts solids with them, but it has no offset of a face or a shell, and so no
//! solid between a sheet and its offset (its README lists "shelling and offsets" among what comes
//! next).
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("no thickening of a sheet: cad-kernel has no offset of a face or a shell".into())
}
