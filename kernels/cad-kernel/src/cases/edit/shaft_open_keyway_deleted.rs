//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with an open keyway cut
//! into it, the box −1 ≤ x ≤ 1, y ≤ −3, z ≥ 8 cut away, running from z = 8 out through the top: its
//! floor y = −3, its walls x = −1 and x = 1 and its end z = 8; all four then deleted, the cylinder
//! closing over the keyway and the top growing back to a whole disc.
//!
//! Refused. cad-kernel builds the keyed shaft: a `prism` of a `disc` cut by a box, a `prism` of a
//! rectangle, from z = 8 up past the top. What is missing is the edit. cad-kernel deletes no face
//! of a solid: nothing in `boolean`, `fillet`, `rounds`, `offset` or `sew` takes faces out of a
//! boundary and runs the faces round them on along their own surfaces until they close the gap, and
//! `offset::normal_sweep` sweeps a face along its normal without extending any face beside it.
//! Joining back the keyway's box would be the result drawn again, not the keyway deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the keyway's floor \
         y = −3, its walls x = ±1 and its end z = 8 out and closes the cylinder of radius 5 over \
         it, the top growing back to a whole disc"
        .into())
}
