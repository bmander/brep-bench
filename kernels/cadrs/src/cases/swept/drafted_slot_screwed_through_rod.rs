//! A rod (the cylinder of radius 5 about the z axis from z = 0 to z = 20) threaded by a bar whose
//! section is a slot (half discs of radius 1 about (12, ±1.5, 0) and the rectangle between them,
//! in the plane x = 12), carried along −x to x = 4 drafted 3° inward, screwed about the z axis,
//! rising 6 a turn, from a turn of −90° to 1350°.
//!
//! The bar is cadrs's solid Loft from the slot at x = 12 to the slot shrunk by 8 tan 3° at x = 4
//! (each side a plane or a half cone); what it passes through (a turn
//! and a slide along its axis at once: a screw) is cut from the rod by cadrs's cut by a swept solid
//! (exactly half a turn at a time where it can be, else meshed); of the bar, its part within 5.5
//! of the axis (the screw maps the cylinder about its axis onto itself, so only that part reaches
//! the rod).
use crate::bench::{arc, p2, p3, v3};
use crate::shared::loft::loft;
use crate::shared::swept::{cut_swept, near_axis, rod, screw, z_axis};
use crate::Refusal;
use cadrs_kernel::{BodyId, Curve2, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    // The plane x = c, sketch x along +y and sketch y along +z (normal +x).
    let at = |c: f64| Plane::new(p3(c, 0.0, 0.0), v3(0.0, 1.0, 0.0), v3(1.0, 0.0, 0.0));
    let slot = |r: f64| {
        Loop::new(vec![
            Curve2::line(p2(-1.5, -r), p2(1.5, -r)),
            arc(p2(1.5, 0.0), r, -90.0, 180.0),
            Curve2::line(p2(1.5, r), p2(-1.5, r)),
            arc(p2(-1.5, 0.0), r, 90.0, 180.0),
        ])
    };
    let shrink = 8.0 * 3f64.to_radians().tan();
    let bar = loft(k, vec![Profile::of(at(4.0), vec![slot(1.0 - shrink)]), Profile::of(at(12.0), vec![slot(1.0)])])?;
    let bar = near_axis(k, bar, 5.5)?;
    let r = rod(k)?;
    cut_swept(k, r, bar, &screw(z_axis(), 6.0, -90.0, 1350.0))
}
