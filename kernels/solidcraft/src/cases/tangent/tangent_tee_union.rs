use crate::solids::*;

pub fn build() -> R<Body> {
    let main = solidcraft_kernel::cylinder(v3(-15.0, 0.0, 0.0), Vec3::X, 5.0, 30.0)?;
    let branch = solidcraft_kernel::cylinder(v3(0.0, -15.0, 2.0), Vec3::Y, 3.0, 30.0)?;
    union(&main, &branch)
}
