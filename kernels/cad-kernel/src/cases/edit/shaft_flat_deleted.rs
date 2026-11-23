//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a flat milled
//! along its whole length, everything with y < −4 cut away; the flat, the strip of the plane y = −4
//! from x = −3 to x = 3, then deleted, the cylinder closing over it and the ends growing back to
//! whole discs.
//!
//! Refused. cad-kernel builds the milled shaft: a `prism` of a `disc` cut by a box, a `prism` of a
//! rectangle, standing on y = −4 and running past both ends. What is missing is the edit.
//! cad-kernel deletes no face of a solid: nothing in `boolean`, `fillet`, `rounds`, `offset` or
//! `sew` takes faces out of a boundary and runs the faces round them on along their own surfaces
//! until they close the gap, and `offset::normal_sweep` sweeps a face along its normal without
//! extending any face beside it. Joining back the slice the flat took would be the result drawn
//! again, not the flat deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the flat y = −4 out \
         and closes the cylinder of radius 5 over it, the two ends growing back to whole discs"
        .into())
}
