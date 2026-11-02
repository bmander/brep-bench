//! A capsule's end, the cylinder of radius 5 from z = 0 to 10 capped by a half ball, shelled inward
//! to a wall 1 thick, open at its hemisphere.
use crate::sketch::{faces_through, capsule};
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let body = capsule(5., 10.)?;
    let open = faces_through(&body, &[[3., 0., 14.]])?;
    shelled(&body, &open, 1., Side::In, Join::Round)
}
