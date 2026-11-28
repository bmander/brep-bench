//! The ball of radius 5 of `geometry/nurbs-sphere`, its whole surface one NURBS face, grown by 2:
//! its one face moved 2 out along its normal, through the poles, onto the surface's offset. The
//! ball has no edge to round. ops's offset of a body's faces, `offset_sheet` (the kernel's offset
//! with the convex edges extended), fails on it with the kernel's bare status 1001, so this is the
//! kernel's offset with them rounded.
//!
//! It stays on raw solidcore's offset (`offset_rounded`, the operation solidcraft-kernel's
//! `offset_solid` runs) because solidcraft-kernel cannot build the input: it has no face on a
//! given NURBS net (its freeform sheets are T-splines, odd degree only, and this net is
//! biquadratic), and its `sew` takes planes, surfaces of revolution and of extrusion, and ruled
//! surfaces, not a NURBS face sewn to itself along a seam and pinched at its poles.
use crate::cases::geometry::nurbs_sphere;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    offset_rounded(&nurbs_sphere::build()?, 2.0)
}
