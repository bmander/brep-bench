//! A torus about the z axis: middle circle radius 5 and tube radius 4.99, its hole 0.01 in radius.
use brep_kernel::{make_torus_brep, BrepSolid, Vec3};

pub fn build() -> Result<BrepSolid, String> {
    make_torus_brep(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), 5.0, 4.99)
}
