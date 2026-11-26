//! The half of the cylinder of radius 5 on the z axis from z = 0 to 10 where y ≥ 0, every one of its
//! eight edges rounded to radius 1: three rounds meet at each of its four corners, one of their
//! faces curved.
//!
//! Rounded in one call (`rounds::rounded`, every edge) the kernel refuses: "a round beside a face
//! on a cylinder: only rounds between planes are" made. Every edge rounded to 1 is the union of
//! every ball of radius 1 inside the half cylinder, so it is the half cylinder shrunk by 1 and then
//! grown by 1, which the kernel's offsets build: the rims' rounds come out as tori, the straight
//! edges' as cylinders and the corners as balls.
use crate::sketch::half_pipe;
use cad_kernel::offset::{grown, shrunk, Join};
use cad_kernel::topo::Brep;

const ROUND: f64 = 1.;

pub fn build() -> Result<Brep, String> {
    grown(&shrunk(&half_pipe()?, ROUND, Join::Round)?, ROUND, Join::Round)
}
