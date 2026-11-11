//! A drilled block shelled open at its bore. The block from the origin to (20, 20, 10) with a hole of
//! radius 3 bored through it along z about x = 10, y = 10, shelled inward to a wall 1 thick, open at
//! the bore: the six flat faces moved 1 inward make the cavity from (1, 1, 1) to (19, 19, 9), which
//! runs out to the bore's cylinder.
//!
//! The drilled block is the square with the hole's circle in it on the Top plane, extruded 10; cadrs's
//! Shell feature (inward), the bore picked as the face to remove.
use crate::bench::{extrude, p3};
use crate::shared::shell::{shell_inward, Joins};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let square = Loop::rectangle(0.0, 0.0, 20.0, 20.0);
    let s = Profile::with_holes(Plane::xy(0.0), square, vec![Loop::circle([10.0, 10.0], 3.0)]);
    let b = extrude(k, &s, 10.0)?;
    shell_inward(k, b, &[p3(13.0, 10.0, 5.0)], 1.0, Joins::Rounded)
}
