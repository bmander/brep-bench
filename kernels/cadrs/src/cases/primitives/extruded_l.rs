//! An L-shaped plate: the polygon through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15)
//! sketched on the Top plane, extruded 8 along +z.
use crate::bench::extrude;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let l = Loop::polygon(&[[0.0, 0.0], [20.0, 0.0], [20.0, 5.0], [5.0, 5.0], [5.0, 15.0], [0.0, 15.0]]);
    extrude(k, &Profile::of(Plane::xy(0.0), vec![l]), 8.0)
}
