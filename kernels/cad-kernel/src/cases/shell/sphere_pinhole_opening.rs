//! The ball of radius 5 about the origin below z = 3.9999, shelled inward to a wall 1 thick, open
//! at its flat face through a pinhole.
use crate::sketch::{faces_through, ball_below};
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let body = ball_below([0., 0., 0.], 5., 3.9999)?;
    let open = faces_through(&body, &[[0.5, 0.5, 3.9999]])?;
    shelled(&body, &open, 1., Side::In, Join::Round)
}
