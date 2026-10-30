//! A circle of radius 2 swept along a quarter of the circle of radius 10 about the z axis in the xy
//! plane, from (10, 0, 0) to (0, 10, 0), square to it, its ends flat.
use crate::solids::SEGMENTS;
use vcad_kernel::vcad_kernel_math::{Point3, Vec3};
use vcad_kernel::vcad_kernel_sketch::SketchProfile;
use vcad_kernel::vcad_kernel_sweep::{CylindricalPath, SweepOptions};
use vcad_kernel::Solid;

const RADIUS: f64 = 2.0;
const ARC_RADIUS: f64 = 10.0;
const TURN: f64 = 90.0;
/// The section's circle is cut into this many arcs, each into SEGMENTS / ARCS lines.
const ARCS: u32 = 4;

pub fn build() -> Result<Solid, crate::Refusal> {
    // vcad's path round the z axis is the cylindrical (cam-track) path, here one that does not
    // rise: knots (angle in degrees, height) at 0° and 90°, turning from +x towards +y. It holds
    // the section in the plane through the axis, which on a level arc is square to it.
    let path = CylindricalPath::from_knots(ARC_RADIUS, vec![(0.0, 0.0), (TURN, 0.0)]);
    // the sweep reads the section in sketch coordinates about the path: a circle about the origin
    let section = SketchProfile::circle(Point3::origin(), Vec3::z(), RADIUS, ARCS);
    // vcad's sweep is faceted: the circle cut into SEGMENTS lines, as a primitive's is; the path
    // is sampled at vcad's own step for a cylindrical path, 0.5°
    let options = SweepOptions { arc_segments: SEGMENTS / ARCS, ..SweepOptions::default() };
    Ok(Solid::sweep_cylindrical(section, &path, options)?)
}
