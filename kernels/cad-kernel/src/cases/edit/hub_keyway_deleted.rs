//! A hub, the tube about the z axis from z = 0 to z = 20 of outer radius 10 and bore of radius 4,
//! with a keyway cut along the bore's whole length, the box −1 ≤ x ≤ 1, −5 ≤ y ≤ 0 cut away: its
//! floor y = −5 and its walls x = −1 and x = 1; the keyway's three faces then deleted, the bore
//! closing over it and the hub's ends growing back to whole rings.
//!
//! Refused. cad-kernel builds the keyed hub: a `prism` of a `disc` of radius 10 cut by a `prism` of
//! a `disc` of radius 4 and a box, a `prism` of a rectangle, running past both ends. What is
//! missing is the edit. cad-kernel deletes no face of a solid: nothing in `boolean`, `fillet`,
//! `rounds`, `offset` or `sew` takes faces out of a boundary and runs the faces round them on along
//! their own surfaces until they close the gap, and `offset::normal_sweep` sweeps a face along its
//! normal without extending any face beside it. Joining back the keyway's box would be the result
//! drawn again, not the keyway deleted.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel deletes no face of a solid: it has no operation that takes the keyway's floor \
         y = −5 and its walls x = ±1 out and closes the bore of radius 4 over it, the hub's two \
         ends growing back to whole rings"
        .into())
}
