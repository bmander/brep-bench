//! The L-shaped plate through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15), 8 high, shelled
//! inward to a wall 1 thick, open at its top, its moved faces extended until they meet.
use crate::sketch::{l_block, faces_through};
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let body = l_block()?;
    let open = faces_through(&body, &[[2., 2., 8.]])?;
    shelled(&body, &open, 1., Side::In, Join::Sharp)
}
