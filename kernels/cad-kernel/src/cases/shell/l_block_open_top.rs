//! The L-shaped plate through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15), 8 high, shelled
//! inward to a wall 1 thick, open at its top.
#[allow(unused_imports)]
use crate::sketch::{block, polygon, faces_through, ball_below, rounded_plate, capsule, drilled};
#[allow(unused_imports)]
use cad_kernel::build::{cone, disc, kept, prism, sphere, torus};
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let body = prism(&polygon(&[[0., 0., 0.], [20., 0., 0.], [20., 5., 0.], [5., 5., 0.], [5., 15., 0.], [0., 15., 0.]]), 0., 8.)?;
    let open = faces_through(&body, &[[2., 2., 8.]])?;
    shelled(&body, &open, 1., Side::In, Join::Round)
}
