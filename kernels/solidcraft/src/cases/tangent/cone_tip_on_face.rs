use crate::solids::*;

pub fn build() -> R<Body> {
    let block = block((-10.0, -10.0, 0.0), (10.0, 10.0, 10.0))?;
    // the kernel has no cone: its half section, apex at the origin, turned about the z axis
    let cone = revolve_z(polygon(&[(0.0, 0.0), (5.0, 15.0), (0.0, 15.0)]), 360.0)?;
    union(&block, &cone)
}
