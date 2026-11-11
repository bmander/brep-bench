//! The regular tetrahedron on (3, 3, 3), (3, −3, −3), (−3, 3, −3), (−3, −3, 3), grown by 1.
use cad_kernel::build::hull;
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let corners = [[3., 3., 3.], [3., -3., -3.], [-3., 3., -3.], [-3., -3., 3.]];
    // the tetrahedron: its corners' convex hull
    grown(&hull(&corners)?, 1., Join::Round)
}
