//! The regular tetrahedron on (3, 3, 3), (3, −3, −3), (−3, 3, −3), (−3, −3, 3), grown by 1.
use cad_kernel::build::hull;
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let tetrahedron = hull(&[[3., 3., 3.], [3., -3., -3.], [-3., 3., -3.], [-3., -3., 3.]])?;
    grown(&tetrahedron, 1., Join::Round)
}
