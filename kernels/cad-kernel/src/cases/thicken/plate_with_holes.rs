//! A sheet: the square in the xy plane from (−10, −10, 0) to (10, 10, 0), with two round holes of
//! radius 3 through it, centred on (−5, 0, 0) and (5, 0, 0). It is one flat face, bounded by the
//! square and the two circles. Thicken it by 1 to each side, so that it is 2 thick in all with the
//! sheet in its middle.
//!
//! Refused. cad-kernel has no thickening of a sheet: it makes sheets (`build::sheet`,
//! `build::sheets`) and cuts solids with them, but it has no offset of a face or a shell, and so no
//! solid between a sheet and its offset (its README lists "shelling and offsets" among what comes
//! next).
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("no thickening of a sheet: cad-kernel has no offset of a face or a shell".into())
}
