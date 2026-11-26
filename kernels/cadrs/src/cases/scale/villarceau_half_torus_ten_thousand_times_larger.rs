//! A solid torus about the z axis, its tube's middle circle of radius 50000 and the tube of radius
//! 30000, cut by the bitangent plane 3x − 4z = 0, keeping what lies on and below it (z ≤ 3x/4):
//! `tangent/villarceau-half-torus` ten thousand times larger.
//!
//! The torus is cadrs's torus primitive, its seam in the plane x = 0 on the side y > 0. What lies
//! above the plane is cut away with a block: a square of side 400000 sketched on the plane itself,
//! extruded 200000 along its normal (−3, 0, 4)/5, away from the kept side, reaching past the torus.
use crate::bench::{cut, extrude, p3, torus, v3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let ring = torus(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 50000.0, 30000.0)?;
    let on = Plane::new(p3(0.0, 0.0, 0.0), v3(4.0, 0.0, 3.0), v3(-3.0, 0.0, 4.0));
    let square = Loop::rectangle(-200000.0, -200000.0, 200000.0, 200000.0);
    let above = extrude(k, &Profile::of(on, vec![square]), 200000.0)?;
    cut(k, ring, above)
}
