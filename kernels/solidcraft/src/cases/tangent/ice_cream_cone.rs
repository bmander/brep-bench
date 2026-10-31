use crate::solids::*;

pub fn build() -> R<Body> {
    let ball = solidcraft_kernel::sphere(Vec3::ZERO, 15.0)?;
    // the kernel has no cone: its half section, apex at (0, 0, −25), turned about the z axis
    let cone = revolve_z(polygon(&[(0.0, -25.0), (12.0, -9.0), (0.0, -9.0)]), 360.0)?;
    union(&ball, &cone)
}
