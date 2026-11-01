//! A sheet: the half of the cylinder of radius 5 about the z axis where y ≥ 0, from z = 0 to z =
//! 10. It is one curved face, bounded by two half circles, at z = 0 and z = 10, and two straight
//! lines, at (5, 0) and (−5, 0). Thicken it by 1 away from the axis.
//!
//! Refused. cad-kernel has no thickening of a sheet: it makes sheets (`build::sheet`,
//! `build::sheets`) and cuts solids with them, but it has no offset of a face or a shell, and so no
//! solid between a sheet and its offset (its README lists "shelling and offsets" among what comes
//! next).
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("no thickening of a sheet: cad-kernel has no offset of a face or a shell".into())
}
