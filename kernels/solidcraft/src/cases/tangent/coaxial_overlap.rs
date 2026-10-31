use crate::solids::*;

pub fn build() -> R<Body> {
    let first = zcyl(0.0, 0.0, 5.0, 0.0, 10.0)?;
    // the kernel's cylinders have their seams at one odd angle: the second turned a quarter turn
    // about the z axis, so the coincident walls are not split in the same place
    let second = solidcraft_kernel::transform(
        &zcyl(0.0, 0.0, 5.0, 6.0, 16.0)?,
        Vec3::ZERO,
        Vec3::ZERO,
        Vec3::Z,
        std::f64::consts::FRAC_PI_2,
    )?;
    union(&first, &second)
}
