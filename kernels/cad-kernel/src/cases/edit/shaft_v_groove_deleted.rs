//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a V-groove turned
//! into it, everything farther from the axis than 3 + |z − 10| cut away: two cones meeting at
//! radius 3 at z = 10, running out to the shaft at z = 8 and z = 12; both cones then deleted
//! together, the shaft's cylinder below and above the groove, one surface, running on across it
//! until it is one face again.
//!
//! Refused. cad-kernel builds the grooved shaft: a `prism` of a `disc` cut by the ring the groove
//! takes, its meridian triangle turned by `build::revolve` (or two `build::cone` frustums cut from
//! it). What is missing is the edit. cad-kernel deletes no face of a solid: nothing in `boolean`,
//! `fillet`, `rounds`, `offset` or `sew` takes faces out of a boundary and runs the faces round
//! them on along their own surfaces until they close the gap, and `offset::normal_sweep` sweeps a
//! face along its normal without extending any face beside it. Joining back the ring the groove
//! took would be the result drawn again, not the groove deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the groove's two \
         cones out and runs the shaft's cylinder on across it from z = 8 to z = 12 until it is one \
         face"
        .into())
}
