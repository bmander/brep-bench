//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with two opposite flats
//! milled along its whole length, everything with |x| > 4 cut away, the strips of the planes x = 4
//! and x = −4 from y = −3 to y = 3; both flats then deleted together, the cylinder closing over
//! both and the ends growing back to whole discs.
//!
//! Refused. cad-kernel builds the milled shaft: a `prism` of a `disc` cut by two boxes, `prism`s of
//! rectangles, standing on x = 4 and x = −4 and running past both ends. What is missing is the
//! edit. cad-kernel deletes no face of a solid: nothing in `boolean`, `fillet`, `rounds`, `offset`
//! or `sew` takes faces out of a boundary and runs the faces round them on along their own surfaces
//! until they close the gap, and `offset::normal_sweep` sweeps a face along its normal without
//! extending any face beside it. Joining back the slices the flats took would be the result drawn
//! again, not the flats deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the flats x = 4 and \
         x = −4 out and closes the cylinder of radius 5 over both, the two ends growing back to \
         whole discs"
        .into())
}
