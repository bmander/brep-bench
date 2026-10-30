//! The rectangle with x from 6 to 10 and z from 0 to 4 in the xz plane, revolved 90° about the z
//! axis, right-handed: from +x towards +y.
use vcad_kernel::vcad_kernel_math::{Point3, Vec3};
use vcad_kernel::vcad_kernel_sketch::SketchProfile;
use vcad_kernel::Solid;

const INNER: f64 = 6.0;
const OUTER: f64 = 10.0;
const HEIGHT: f64 = 4.0;
const TURN: f64 = 90.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    // a sketch in the xz plane: its x along x and its y along z
    let section = SketchProfile::rectangle(
        Point3::new(INNER, 0.0, 0.0),
        Vec3::x(),
        Vec3::z(),
        OUTER - INNER,
        HEIGHT,
    );
    // a positive angle about +z turns right-handed, from +x towards +y
    Ok(Solid::revolve(section, Point3::origin(), Vec3::z(), TURN)?)
}
