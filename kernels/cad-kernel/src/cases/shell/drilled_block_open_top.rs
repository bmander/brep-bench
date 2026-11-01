//! The block from the origin to (20, 20, 10) bored through along x = 10, y = 10 with radius 3,
//! shelled inward to a wall 1 thick, open at its top.
#[allow(unused_imports)]
use crate::sketch::{block, polygon, faces_through, ball_below, rounded_plate, capsule, drilled};
#[allow(unused_imports)]
use cad_kernel::build::{cone, disc, kept, prism, sphere, torus};
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let body = drilled(10.)?;
    let open = faces_through(&body, &[[2., 2., 10.]])?;
    shelled(&body, &open, 1., Side::In, Join::Round)
}
