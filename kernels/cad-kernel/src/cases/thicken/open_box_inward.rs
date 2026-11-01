//! A sheet: five faces of the cube from the origin to (10, 10, 10), its floor at z = 0 and its four
//! walls, with no top: an open tray, its boundary the square of the walls' top edges at z = 10.
//! Thicken it by 1 into the cube. The moved faces cross near the tray's edges and corners, and each
//! is trimmed where it meets the others.
//!
//! Refused. cad-kernel has no thickening of a sheet: it makes sheets (`build::sheet`,
//! `build::sheets`) and cuts solids with them, but it has no offset of a face or a shell, and so no
//! solid between a sheet and its offset (its README lists "shelling and offsets" among what comes
//! next).
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("no thickening of a sheet: cad-kernel has no offset of a face or a shell".into())
}
