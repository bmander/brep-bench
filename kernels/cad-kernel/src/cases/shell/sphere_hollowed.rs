//! A ball of radius 5 about the origin hollowed to a wall 1 thick, closed.
#[allow(unused_imports)]
use crate::sketch::{block, polygon, faces_through, ball_below, rounded_plate, capsule, drilled};
#[allow(unused_imports)]
use cad_kernel::build::{cone, disc, kept, prism, sphere, torus};
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let body = sphere([0., 0., 0.], [0., 0., 1.], 5.)?;
    let open = faces_through(&body, &[])?;
    shelled(&body, &open, 1., Side::In, Join::Round)
}
