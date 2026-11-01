//! A sheet: the side of the cone frustum about the z axis whose radius runs from 6 at z = 0 to 3 at
//! z = 4 (5 along its slant), one face bounded by those two circles. Thicken it by 1 along its
//! normal away from the axis. That normal leans up: in a section through the axis it is (4/5, 3/5),
//! 4/5 outward and 3/5 up.
//!
//! Refused. cad-kernel has no thickening of a sheet: it makes sheets (`build::sheet`,
//! `build::sheets`) and cuts solids with them, but it has no offset of a face or a shell, and so no
//! solid between a sheet and its offset (its README lists "shelling and offsets" among what comes
//! next).
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("no thickening of a sheet: cad-kernel has no offset of a face or a shell".into())
}
