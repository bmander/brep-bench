//! `tangent/villarceau-half-torus` ten thousand times larger: the solid torus about the z axis
//! (middle circle of radius 50000, tube of radius 30000), cut by the bitangent plane 3x − 4z = 0,
//! keeping what lies on and below it, z ≤ 3x/4. The kernel's torus about z turns its tube's
//! section from +y, the seam in the plane x = 0 on the side y > 0. Built as the original is,
//! every length times 10000.
use crate::solids::p;
use crate::solids::singular::keep_toward;
use crate::Refusal;
use brep_kernel::{make_torus_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let torus = make_torus_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 50000.0, 30000.0)?;
    // the side 3x − 4z ≥ 0 is z ≤ 3x/4
    keep_toward(&torus, p(0.0, 0.0, 0.0), p(3.0, 0.0, -4.0), 200000.0)
}
