//! The pentagon through (0, 0), (20, 0), (20, 7), (17, 10) and (0, 10) in the plane y = 0, corners
//! as (x, z), extruded 10 along +y: a block with one edge beveled. Then the bevel deleted, the top
//! and the side run on to meet: the box from the origin to (20, 10, 10).
//!
//! The pentagon is extruded; then Delete face takes the bevel.
use crate::bench::{extrude, p3};
use crate::shared::edit::deleted;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Pick, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let section = Loop::polygon(&[[0.0, 0.0], [20.0, 0.0], [20.0, 7.0], [17.0, 10.0], [0.0, 10.0]]);
    // the Front plane's normal is −y: extrude against it, along +y
    let beveled = extrude(k, &Profile::of(Plane::xz(0.0), vec![section]), -10.0)?;
    let bevel = k.faces_at(beveled, &[p3(18.5, 5.0, 8.5)])?;
    deleted(k, beveled, &bevel)
}
