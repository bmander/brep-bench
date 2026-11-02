//! The block from the origin to (20, 20, 10) bored through along x = 15.5, y = 10 with radius 3,
//! shelled inward to a wall 1 thick, open at its top.
use crate::sketch::{faces_through, drilled};
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let body = drilled(15.5)?;
    let open = faces_through(&body, &[[2., 2., 10.]])?;
    shelled(&body, &open, 1., Side::In, Join::Round)
}
