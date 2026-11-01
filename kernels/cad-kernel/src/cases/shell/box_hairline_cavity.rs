//! The box from the origin to (10, 8, 6) shelled inward to a wall 3.99 thick, open at its top.
use crate::sketch::{block, faces_through};
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let body = block([0., 0., 0.], [10., 8., 6.])?;
    let open = faces_through(&body, &[[5., 4., 6.]])?;
    shelled(&body, &open, 3.99, Side::In, Join::Round)
}
