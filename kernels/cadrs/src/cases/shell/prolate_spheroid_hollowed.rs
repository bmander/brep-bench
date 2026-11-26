//! The solid prolate spheroid (x² + y²)/9 + z²/36 ≤ 1 about the z axis, hollowed to a closed
//! wall 1 thick with no opening: the cavity bounded by the spheroid's surface moved 1 inward
//! along its normals (not an ellipsoid).
//!
//! The spheroid built as `geometry/oblate-spheroid` builds its own, the half ellipse revolved a
//! whole turn about z: here sketched on the plane through the z axis whose sketch x runs along
//! the model's z and sketch y along its x, the ellipse's major semi-axis 6 along sketch x (cadrs
//! turns a revolved ellipse whose given first semi-axis is the shorter one inside out), its half
//! with x ≥ 0 closed by the axis. Then cadrs's Shell feature with Hollow set (no face removed).
use crate::bench::{p2, p3, revolve, v3};
use crate::shared::geometry::ellipse_arc;
use crate::shared::shell::hollow;
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, Curve2, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let on = Plane::new(p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), v3(0.0, 1.0, 0.0));
    let half = Loop::new(vec![
        ellipse_arc((0.0, 0.0), 6.0, 3.0, 0.0, 180.0),
        Curve2::line(p2(-6.0, 0.0), p2(6.0, 0.0)),
    ]);
    let s = Profile::of(on, vec![half]);
    let spheroid = revolve(k, &s, Axis::new(p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0)), None)?;
    hollow(k, spheroid, 1.0)
}
