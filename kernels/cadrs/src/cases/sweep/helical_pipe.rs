//! A pipe of radius 1 along the helix of radius 10 about the z axis rising 5 a turn, from
//! (10, 0, 0) three right-handed turns, the section square to the path, the ends flat: the
//! circle drawn on the plane through (10, 0, 0) square to the helix there, swept along the helix
//! as cadrs's Helix feature draws it (rational Bézier pieces on its cylinder, which the sweep
//! follows by the helix's screw motion).
use crate::bench::p3;
use crate::shared::sweep::{helix, helix_at, sweep};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let o = p3(0.0, 0.0, 0.0);
    let (start, tangent) = helix_at(o, 10.0, 5.0, 0.0, 0.0);
    let section = Profile::of(Plane::square_to(start, tangent), vec![Loop::circle([0.0, 0.0], 1.0)]);
    sweep(k, &section, helix(o, 10.0, 5.0, 0.0, 3.0))
}
