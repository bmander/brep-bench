//! The cylinder of radius 6 on the z axis from z = 0 to z = 8 with a cone frustum on it, from
//! radius 6 at z = 8 to radius 4 at z = 10: a cylinder with its top rim beveled. Then the bevel,
//! the frustum's side, deleted, the top and the side run on to meet: the cylinder of radius 6 from
//! z = 0 to z = 10.
//!
//! The cylinder and the frustum are united; then Delete face takes the bevel.
use crate::bench::{cone, cylinder, p3, union, v3};
use crate::shared::edit::deleted;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let z = v3(0.0, 0.0, 1.0);
    let c = cylinder(k, p3(0.0, 0.0, 0.0), z, 6.0, 8.0)?;
    let frustum = cone(k, p3(0.0, 0.0, 8.0), z, 6.0, 4.0, 2.0)?;
    let beveled = union(k, c, frustum)?;
    let bevel = k.faces_at(beveled, &[p3(0.0, -5.0, 9.0)])?;
    deleted(k, beveled, &bevel)
}
