//! The plate 20 × 14, 10 high, its vertical edges rounded to 2, shelled inward to a wall 2 thick,
//! open at its top: the rounds shrunk to edges.
#[allow(unused_imports)]
use crate::sketch::{block, polygon, faces_through, ball_below, rounded_plate, capsule, drilled};
#[allow(unused_imports)]
use cad_kernel::build::{cone, disc, kept, prism, sphere, torus};
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let body = rounded_plate([0., 0.], [20., 14.], 2., 10.)?;
    let open = faces_through(&body, &[[10., 7., 10.]])?;
    shelled(&body, &open, 2., Side::In, Join::Round)
}
