//! A sheet: the rectangle in the xy plane from (0, 0, 0) to (10, 8, 0), a single flat face with no
//! solid behind it, thickened by 2 to its +z side. The rectangle sketched on the Top plane and
//! thickened as a sketch region, its face's normal along the plane's (+z).
use crate::shared::thicken::thicken_sketch;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let s = Profile::of(Plane::xy(0.0), vec![Loop::rectangle(0.0, 0.0, 10.0, 8.0)]);
    thicken_sketch(k, s, 2.0, 0.0)
}
