//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a ring groove
//! turned into it, everything between radius 4 and 5 from z = 8 to z = 12 cut away: a floor, the
//! cylinder of radius 4, and two walls, the flat rings at z = 8 and z = 12; the floor and both
//! walls then deleted together, the shaft's cylinder below and above the groove, one surface,
//! running on across it until it is one face again.
//!
//! Refused. cad-kernel builds the grooved shaft: a `prism` of a `disc` cut by a ring, a `prism` of
//! a `disc` of radius 6 less one of radius 4 from z = 8 to 12; or the shaft's meridian notched and
//! turned by `build::revolve`. What is missing is the edit. cad-kernel deletes no face of a solid:
//! nothing in `boolean`, `fillet`, `rounds`, `offset` or `sew` takes faces out of a boundary and
//! runs the faces round them on along their own surfaces until they close the gap, and
//! `offset::normal_sweep` sweeps a face along its normal without extending any face beside it.
//! Joining back the ring the groove took would be the result drawn again, not the groove deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the groove's floor \
         (the cylinder of radius 4) and its walls at z = 8 and z = 12 out and runs the shaft's \
         cylinder on across the groove until it is one face"
        .into())
}
