//! The solid oblate spheroid (x² + y²)/36 + z²/9 ≤ 1 grown by 1: one smooth face, the spheroid's
//! surface moved 1 out along its normals, which is no ellipsoid.
use crate::cases::geometry::oblate_spheroid;
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    grown(&oblate_spheroid::build()?, 1., Join::Round)
}
