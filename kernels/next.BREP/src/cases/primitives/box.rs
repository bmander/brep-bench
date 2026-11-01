//! A block 10 × 20 × 30 from the origin to (10, 20, 30).
use brep_kernel::{make_box_brep, BrepSolid, Vec3};

pub fn build() -> Result<BrepSolid, String> {
    make_box_brep(Vec3::new(0.0, 0.0, 0.0), 10.0, 20.0, 30.0)
}
