//! The lower half of the ball of radius 5 about the origin shelled inward to a wall 1 thick, open
//! at its flat face.
#[allow(unused_imports)]
use crate::sketch::{block, polygon, faces_through, ball_below, rounded_plate, capsule, drilled};
#[allow(unused_imports)]
use cad_kernel::build::{cone, disc, kept, prism, sphere, torus};
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let body = ball_below([0., 0., 0.], 5., 0.)?;
    let open = faces_through(&body, &[[1., 1., 0.]])?;
    shelled(&body, &open, 1., Side::In, Join::Round)
}
