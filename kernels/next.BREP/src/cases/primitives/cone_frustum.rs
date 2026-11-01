//! A frustum on the z axis, of radius 5 at z = 0 and 2 at z = 6.
use brep_kernel::{make_cone_brep, BrepSolid, Vec3};

pub fn build() -> Result<BrepSolid, String> {
    make_cone_brep(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), 5.0, 2.0, 6.0)
}
