//! The cone frustum about the z axis of radius 6 at z = 0 narrowing to radius 3 at z = 20, a flat
//! milled along its side, everything with y < −4 cut away, the flat running up from the base to
//! where the cone narrows past it, its edge on the cone a hyperbola; the flat then deleted, the
//! cone closing over it and the base growing back to a whole disc.
//!
//! Refused. cad-kernel builds the milled frustum: `build::cone` cut by a box, a `prism` of a
//! rectangle, standing on y = −4. What is missing is the edit. cad-kernel deletes no face of a
//! solid: nothing in `boolean`, `fillet`, `rounds`, `offset` or `sew` takes faces out of a boundary
//! and runs the faces round them on along their own surfaces until they close the gap, and
//! `offset::normal_sweep` sweeps a face along its normal without extending any face beside it.
//! Joining back the slice the flat took would be the result drawn again, not the flat deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the flat y = −4 out \
         and closes the cone over its hyperbolic edge, the base growing back to a whole disc of \
         radius 6"
        .into())
}
