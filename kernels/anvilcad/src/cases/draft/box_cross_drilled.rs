//! The box from the origin to (30, 20, 20) drilled through along x by a hole of radius 4 on the
//! line y = 10, z = 10, then its faces x = 0 and x = 30 drafted 8° about the floor, pulling along
//! +z.
use anvil_brep::Solid;

use crate::shared::{block, draft, rod, subtract};

pub fn build() -> Result<Solid, String> {
    let drilled = subtract(&block([0., 0., 0.], [30., 20., 20.])?, &rod([-1., 10., 10.], [31., 10., 10.], 4.)?)?;
    draft(&drilled, &[[0., 2., 2.], [30., 2., 2.]], [0., 0., 0.], [0., 0., 1.], 8.)
}
