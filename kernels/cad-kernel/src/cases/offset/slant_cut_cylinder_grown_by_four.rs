//! The cylinder of radius 3 on the z axis from its flat bottom at z = −10 up to where the plane
//! z = x·tan 45° cuts it, grown by 4 with rounded edges: every point within 4 of it.
//!
//! 4 is more than the ellipse's least radius of curvature, b²/a = 2.121320 at the ends of its
//! long axis; the round along the ellipse is only the part of the tube of radius 4 between the
//! top's normal and the side's, on the ellipse's outer side, where it stays smooth.
//!
//! Refused, by the kernel: "the join of face 0 (cylinder) and `far` (plane): the ellipse turns on
//! a radius of 2.1213… at [3, 0, 3], tighter than the tube's 4: the tube would fold".
//! `offset::grown` joins a solid's convex crease by the whole tube about it (`offset::tube`, along
//! an ellipse `swept::canal::ring`), whose curvature row refuses any curve turning tighter than the
//! distance; the wedge of the tube between the two faces' normals (`offset::wedge`) is built only
//! along a line or a circle. Growing in two steps (by 2, then by the rest) is refused too: the
//! first step's round is a face family, which `offset::normal_sweep` does not move ("a family has
//! no offset in closed form").
use cad_kernel::build::{disc, kept, prism};
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

const RADIUS: f64 = 3.;
const BOTTOM: f64 = -10.;
const TILT_DEGREES: f64 = 45.;
const GROWTH: f64 = 4.;

pub fn build() -> Result<Brep, String> {
    // a cylinder reaching up past where the plane leaves it, at most 3 above the origin
    let cylinder = prism(&disc([0., 0., BOTTOM], [0., 0., 1.], RADIUS), 0., 2. * -BOTTOM)?;
    // kept below the plane through the origin that leans at the tilt about y, on the side of its
    // downward normal
    let tilt = TILT_DEGREES.to_radians();
    let slanted = kept(&cylinder, [0., 0., 0.], [tilt.sin(), 0., -tilt.cos()], 0.)?;
    grown(&slanted, GROWTH, Join::Round)
}
