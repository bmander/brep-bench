//! The ball of radius 5 about the origin below z = 3, shelled inward to a wall 1 thick, open at its
//! flat face.
use crate::sketch::{faces_through, ball_below};
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let body = ball_below([0., 0., 0.], 5., 3.)?;
    let open = faces_through(&body, &[[1., 1., 3.]])?;
    shelled(&body, &open, 1., Side::In, Join::Round)
}
