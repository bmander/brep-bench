//! Refused. The bar's side is the spline's surface of extrusion along y, and it meets the screw's
//! motion square-on at a point of each of its rulings found only numerically, on a curve in no
//! plane; the groove's walls are what that curve sweeps. So no section of the bar screwed by the
//! kernel's helical sweep (`sweep_helix`) carries it. solidcraft-kernel has no sweep of a solid and
//! no envelope, and that curve fitted here would be geometry made outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("no layer (solidcraft-kernel, solidcore-ops, solidcore) has a sweep of a solid or an envelope: the spline bar's side meets \
         the screw square-on along a curve found only numerically on its rulings, in no plane, \
         so no planar section screwed by any layer's screw sweep (the kernel's `sweep_helix`, ops's `screw_sweep`) makes the groove's walls"
        .into())
}
