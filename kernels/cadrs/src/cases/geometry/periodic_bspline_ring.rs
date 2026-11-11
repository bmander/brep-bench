//! A ring about the z axis, its section a closed, periodic B-spline curve turned a whole turn
//! about the axis. The section, in the xz plane, is the periodic uniform quadratic B-spline whose
//! control polygon is the square of corners (13, 0, 3), (7, 0, 3), (7, 0, −3) and (13, 0, −3):
//! four parabolic arcs, each from the middle of one side to the middle of the next ((13, 0, 0)
//! to (10, 0, 3), and so on), with the square's corner as its control point.
//!
//! The section is that periodic B-spline, one sketch spline on the Front (xz) plane
//! (`Curve2::Spline`, its four poles and the first two again, its knots evenly spaced),
//! revolved a whole turn about z.
use crate::shared::geometry::{turned_about_z, Section};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Plane};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let section =
        Section::on(Plane::xz(0.0)).periodic(2, &[(13.0, 3.0), (7.0, 3.0), (7.0, -3.0), (13.0, -3.0)]);
    turned_about_z(k, &section)
}
