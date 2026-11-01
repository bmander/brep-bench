//! A sheet: five faces of the cube from the origin to (10, 10, 10), its floor at z = 0 and its four
//! walls, with no top: an open tray, its boundary the square of the walls' top edges at z = 10.
//! Thicken it by 1 out of the cube, with its edges rounded: each face moves 1 along its normal, the
//! gap the moved faces leave at each of the tray's eight edges (the floor's four and the four
//! upright ones between walls) is closed by the quarter of the cylinder of radius 1 about the edge
//! that lies between them, and the gap left at each of the floor's four corners by the eighth of
//! the ball of radius 1 about the corner that lies between those.
//!
//! Refused. cad-kernel has no thickening of a sheet: it makes sheets (`build::sheet`,
//! `build::sheets`) and cuts solids with them, but it has no offset of a face or a shell, and so no
//! solid between a sheet and its offset (its README lists "shelling and offsets" among what comes
//! next).
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("no thickening of a sheet: cad-kernel has no offset of a face or a shell".into())
}
