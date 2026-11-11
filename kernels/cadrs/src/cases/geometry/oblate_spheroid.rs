//! A solid oblate spheroid centered on the origin, about the z axis: the ellipsoid
//! (x² + y²)/36 + z²/9 ≤ 1, of radius 6 at its equator and reaching to z = ±3 at its poles.
//!
//! Built as the description puts it: on the Front (xz) plane, the half of the ellipse
//! x²/36 + z²/9 = 1 with x ≥ 0 (an ellipse arc, semi-axes 6 along x and 3 along z, from the
//! south pole to the north) closed by the z axis, revolved a whole turn about z.
use crate::bench::{p2, p3, revolve, v3};
use crate::shared::geometry::ellipse_arc;
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, Curve2, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let half = Loop::new(vec![
        ellipse_arc((0.0, 0.0), 6.0, 3.0, -90.0, 180.0),
        Curve2::line(p2(0.0, 3.0), p2(0.0, -3.0)),
    ]);
    let s = Profile::of(Plane::xz(0.0), vec![half]);
    revolve(k, &s, Axis::new(p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0)), None)
}
