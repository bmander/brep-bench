//! A torus about the z axis through the origin: major radius 10, tube radius 2.
use brep_kernel::{make_torus_brep, BrepSolid, Vec3};

pub fn build() -> Result<BrepSolid, String> {
    make_torus_brep(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), 10.0, 2.0)
}
