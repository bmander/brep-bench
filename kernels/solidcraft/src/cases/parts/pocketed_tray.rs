use crate::solids::*;

pub fn build() -> R<Body> {
    let blank = block((-36.0, 0.0, -24.0), (36.0, 16.0, 24.0))?;
    let mut tray = cut(&blank, &block((-33.0, 0.0, -21.0), (33.0, 13.0, 21.0))?)?;
    for (x, z) in [(26.0, 14.0), (-26.0, 14.0), (-26.0, -14.0), (26.0, -14.0)] {
        // the standoff, a tube from y = 5 to the floor at y = 13, its hole as long as it is
        let post = solidcraft_kernel::cylinder(v3(x, 5.0, z), Vec3::Y, 5.0, 8.0)?;
        let hole = solidcraft_kernel::cylinder(v3(x, 5.0, z), Vec3::Y, 2.0, 8.0)?;
        tray = union(&tray, &cut(&post, &hole)?)?;
    }
    Ok(tray)
}
