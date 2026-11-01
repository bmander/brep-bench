//! The box from the origin to (10, 8, 6) shelled inward to a wall 1 thick, open at its top and
//! bottom.
use crate::sketch::{block, faces_through};
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let body = block([0., 0., 0.], [10., 8., 6.])?;
    let open = faces_through(&body, &[[5., 4., 6.], [5., 4., 0.]])?;
    shelled(&body, &open, 1., Side::In, Join::Round)
}
