//! The solid oblate spheroid (x² + y²)/36 + z²/9 ≤ 1 hollowed to a closed wall 1 thick: the cavity
//! bounded by the spheroid's surface moved 1 inward along its normals, which is no ellipsoid.
use crate::cases::geometry::oblate_spheroid;
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    shelled(&oblate_spheroid::build()?, &[], 1., Side::In, Join::Round)
}
