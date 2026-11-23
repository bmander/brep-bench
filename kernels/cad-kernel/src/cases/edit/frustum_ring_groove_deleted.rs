//! The cone frustum about the z axis of radius 6 at z = 0 narrowing to radius 3 at z = 20, a ring
//! groove turned into it, everything outside radius 4 from z = 8 to z = 12 cut away: a floor, the
//! cylinder of radius 4, and two walls, the flat rings at z = 8 and z = 12; the floor and both
//! walls then deleted together, the cone below the groove and the cone above it, one surface,
//! running on across it until they are one face again.
//!
//! Refused. cad-kernel builds the grooved frustum: `build::cone` cut by a ring, a `prism` of a
//! `disc` of radius 4.8 (past the cone at z = 8) less a `prism` of a `disc` of radius 4, from z = 8
//! to 12; or the frustum's meridian notched and turned by `build::revolve`. What is missing is the
//! edit. cad-kernel deletes no face of a solid: nothing in `boolean`, `fillet`, `rounds`, `offset`
//! or `sew` takes faces out of a boundary and runs the faces round them on along their own surfaces
//! until they close the gap, and `offset::normal_sweep` sweeps a face along its normal without
//! extending any face beside it. Joining back the ring the groove took would be the result drawn
//! again, not the groove deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the groove's floor \
         (the cylinder of radius 4) and its walls at z = 8 and z = 12 out and runs the cone on \
         across the groove until the cone below and above it is one face"
        .into())
}
