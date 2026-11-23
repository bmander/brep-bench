//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a round-bottomed
//! groove turned into it, the torus whose tube of radius 1.5 runs round the circle of radius 5 at z
//! = 10 cut away: one face, the torus's inner half from z = 8.5 to z = 11.5; that face then
//! deleted, the shaft's cylinder below and above the groove, one surface, running on across it
//! until it is one face again.
//!
//! Refused. cad-kernel builds the grooved shaft: a `prism` of a `disc` cut by `build::torus`. What
//! is missing is the edit. cad-kernel deletes no face of a solid: nothing in `boolean`, `fillet`,
//! `rounds`, `offset` or `sew` takes faces out of a boundary and runs the faces round them on along
//! their own surfaces until they close the gap, and `offset::normal_sweep` sweeps a face along its
//! normal without extending any face beside it. Joining the torus back in would be the result drawn
//! again, not the groove deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the groove's half \
         torus out and runs the shaft's cylinder on across it from z = 8.5 to z = 11.5 until it is \
         one face"
        .into())
}
