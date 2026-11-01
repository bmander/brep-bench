//! The box from the origin to (10, 8, 6) shelled inward to a wall 1 thick, open at every face but
//! its bottom.
#[allow(unused_imports)]
use crate::sketch::{block, polygon, faces_through, ball_below, rounded_plate, capsule, drilled};
#[allow(unused_imports)]
use cad_kernel::build::{cone, disc, kept, prism, sphere, torus};
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let body = block([0., 0., 0.], [10., 8., 6.])?;
    let open = faces_through(&body, &[[5., 4., 6.], [0., 4., 3.], [10., 4., 3.], [5., 0., 3.], [5., 8., 3.]])?;
    shelled(&body, &open, 1., Side::In, Join::Round)
}
