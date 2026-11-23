//! The ball of radius 5 about the origin with its top cut flat, everything above z = 3 cut away,
//! leaving a flat disc of radius 4 at z = 3; the flat then deleted, the ball closing over the
//! circle of radius 4 its edge is.
//!
//! Refused. cad-kernel builds the flattened ball: `build::sphere` cut by a `prism` of a `disc`
//! standing on z = 3 (or `build::kept` below that plane). What is missing is the edit. cad-kernel
//! deletes no face of a solid: nothing in `boolean`, `fillet`, `rounds`, `offset` or `sew` takes
//! faces out of a boundary and runs the faces round them on along their own surfaces until they
//! close the gap, and `offset::normal_sweep` sweeps a face along its normal without extending any
//! face beside it. Joining back the cap the flat took would be the result drawn again, not the flat
//! deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the flat at z = 3 \
         out and closes the ball over the circle of radius 4 its edge is"
        .into())
}
