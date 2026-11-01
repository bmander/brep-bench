//! A sheet: the half of the cylinder of radius 5 about the z axis where y ≥ 0, from z = 0 to z =
//! 10, one curved face bounded by two half circles and two straight lines. Thicken it by 2 toward
//! the axis: to the sheet's concave side, where its moved face is smaller than itself.
//!
//! Refused. cad-kernel has no thickening of a sheet: it makes sheets (`build::sheet`,
//! `build::sheets`) and cuts solids with them, but it has no offset of a face or a shell, and so no
//! solid between a sheet and its offset (its README lists "shelling and offsets" among what comes
//! next).
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("no thickening of a sheet: cad-kernel has no offset of a face or a shell".into())
}
