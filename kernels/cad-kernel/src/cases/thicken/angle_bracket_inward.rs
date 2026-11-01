//! A sheet of two flat faces meeting at a right angle along a crease: the rectangle in the xy plane
//! from (0, 0, 0) to (10, 8, 0), the floor, and the rectangle in the yz plane from (0, 0, 0) to (0,
//! 8, 10), the wall. They share the edge from (0, 0, 0) to (0, 8, 0).
//!
//! Refused. cad-kernel has no thickening of a sheet: it makes sheets (`build::sheet`,
//! `build::sheets`) and cuts solids with them, but it has no offset of a face or a shell, and so no
//! solid between a sheet and its offset (its README lists "shelling and offsets" among what comes
//! next).
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("no thickening of a sheet: cad-kernel has no offset of a face or a shell".into())
}
