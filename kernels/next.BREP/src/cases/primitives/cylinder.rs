//! A cylinder of radius 5 on the z axis, from z = 0 to z = 10.
use brep_kernel::{make_cylinder_brep, BrepSolid, Vec3};

pub fn build() -> Result<BrepSolid, String> {
    make_cylinder_brep(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), 5.0, 10.0)
}
