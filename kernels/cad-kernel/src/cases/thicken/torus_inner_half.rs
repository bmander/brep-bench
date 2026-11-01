//! A sheet: the torus about the z axis, centred on the origin, whose tube has radius 2 about a
//! middle circle of radius 6, and of its surface only the inner half: the points of it no farther
//! than 6 from the z axis. It is one face, saddle-shaped everywhere, bounded by the circles of
//! radius 6 at z = 2 and z = −2, the top and bottom of the tube. Thicken it by 1 away from the
//! middle circle, toward the z axis.
//!
//! Refused. cad-kernel has no thickening of a sheet: it makes sheets (`build::sheet`,
//! `build::sheets`) and cuts solids with them, but it has no offset of a face or a shell, and so no
//! solid between a sheet and its offset (its README lists "shelling and offsets" among what comes
//! next).
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("no thickening of a sheet: cad-kernel has no offset of a face or a shell".into())
}
