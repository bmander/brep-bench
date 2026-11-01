//! The plate 20 × 14, 10 high, its vertical edges rounded to 2, shelled inward to a wall 3 thick,
//! open at its top: the rounds consumed.
use crate::sketch::{faces_through, rounded_plate};
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let body = rounded_plate([0., 0.], [20., 14.], 2., 10.)?;
    let open = faces_through(&body, &[[10., 7., 10.]])?;
    shelled(&body, &open, 3., Side::In, Join::Round)
}
