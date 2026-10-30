//! The rectangle with x from 6 to 10 and z from 0 to 4 in the xz plane, revolved a full turn about
//! the z axis.
use vcad_kernel::vcad_kernel_math::{Point3, Vec3};
use vcad_kernel::vcad_kernel_sketch::SketchProfile;
use vcad_kernel::Solid;

const INNER: f64 = 6.0;
const OUTER: f64 = 10.0;
const HEIGHT: f64 = 4.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    // a sketch in the xz plane: its x along x and its y along z
    let section = SketchProfile::rectangle(
        Point3::new(INNER, 0.0, 0.0),
        Vec3::x(),
        Vec3::z(),
        OUTER - INNER,
        HEIGHT,
    );
    Ok(Solid::revolve(section, Point3::origin(), Vec3::z(), 360.0)?)
}
