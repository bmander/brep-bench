//! A sheet: the whole cylinder of radius 5 about the z axis from z = 0 to z = 10, one face that
//! closes around on itself, bounded only by its two circles, at z = 0 and z = 10. A kernel may
//! store the face with a seam where its parameter wraps around; the seam is no edge of the sheet,
//! and leaves no wall. Thicken it by 1 away from the axis.
//!
//! Refused. cad-kernel has no thickening of a sheet: it makes sheets (`build::sheet`,
//! `build::sheets`) and cuts solids with them, but it has no offset of a face or a shell, and so no
//! solid between a sheet and its offset (its README lists "shelling and offsets" among what comes
//! next).
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("no thickening of a sheet: cad-kernel has no offset of a face or a shell".into())
}
