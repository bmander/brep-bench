//! The ellipse x²/25 + y²/9 ≤ 1 of the plane z = 0 extruded up to z = 10. next.BREP has no
//! ellipse among its curves, so it is the round cylinder of radius 1 and height 10 stretched by 5
//! along x and 3 along y: an affine map, which the kernel applies exactly to its rational poles.
use crate::solids::geometry::stretch;
use brep_kernel::{make_cylinder_brep, transform_brep, BrepSolid, Vec3};

pub fn build() -> Result<BrepSolid, String> {
    let round = make_cylinder_brep(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), 1.0, 10.0)?;
    transform_brep(&round, stretch(5.0, 3.0, 1.0)?, false)
}
