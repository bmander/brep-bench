//! The half with y ≥ 0 of the torus of radius 2 about a middle circle of radius 6, shelled inward
//! to a wall 0.5 thick, open at both flat faces.
#[allow(unused_imports)]
use crate::sketch::{block, polygon, faces_through, ball_below, rounded_plate, capsule, drilled};
#[allow(unused_imports)]
use cad_kernel::build::{cone, disc, kept, prism, sphere, torus};
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let body = kept(&torus([0., 0., 0.], [0., 0., 1.], 6., 2.)?, [0., 0., 0.], [0., 1., 0.], 0.)?;
    let open = faces_through(&body, &[[6.5, 0., 0.5], [-6.5, 0., 0.5]])?;
    shelled(&body, &open, 0.5, Side::In, Join::Round)
}
