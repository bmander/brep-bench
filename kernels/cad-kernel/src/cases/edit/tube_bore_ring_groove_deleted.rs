//! A tube about the z axis from z = 0 to z = 20, outer radius 10 and bore of radius 4, with a ring
//! groove turned into the bore, everything inside radius 6 from z = 8 to z = 12 cut away: a floor,
//! the cylinder of radius 6, and two walls, the flat rings from radius 4 to 6 at z = 8 and z = 12;
//! the floor and both walls then deleted together, the bore below and above the groove, one
//! surface, running on across it until it is one face again.
//!
//! Refused. cad-kernel builds the grooved tube: a `prism` of a `disc` of radius 10 cut by a `prism`
//! of a `disc` of radius 4 right through and one of radius 6 from z = 8 to 12; or the tube's
//! meridian notched and turned by `build::revolve`. What is missing is the edit. cad-kernel deletes
//! no face of a solid: nothing in `boolean`, `fillet`, `rounds`, `offset` or `sew` takes faces out
//! of a boundary and runs the faces round them on along their own surfaces until they close the
//! gap, and `offset::normal_sweep` sweeps a face along its normal without extending any face beside
//! it. Joining back the ring the groove took would be the result drawn again, not the groove
//! deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the groove's floor \
         (the cylinder of radius 6) and its walls at z = 8 and z = 12 out and runs the bore of \
         radius 4 on across the groove until it is one face"
        .into())
}
