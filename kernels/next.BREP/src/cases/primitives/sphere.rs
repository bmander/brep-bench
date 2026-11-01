//! A ball of radius 5 about the origin.
use brep_kernel::{make_sphere_brep, BrepSolid, Vec3};

pub fn build() -> Result<BrepSolid, String> {
    make_sphere_brep(Vec3::new(0.0, 0.0, 0.0), 5.0, Vec3::new(0.0, 0.0, 1.0))
}
