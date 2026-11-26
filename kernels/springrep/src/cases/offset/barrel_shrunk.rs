//! The ball of radius 5 about the origin between the planes z = −3 and z = 3, shrunk by 1:
//! springrep's own offset by −1.
use springrep::boolean::intersection;
use springrep::topo::Brep;

use crate::shared::boolean::{ball, between};
use crate::shared::e;
use crate::shared::offset::shrunk;

pub fn build() -> Result<Brep, String> {
    let whole = ball([0., 0., 0.], 5., "ball")?;
    // the slab between the planes, wider than the ball
    let slab = between([-6., -6., -3.], [6., 6., 3.], "slab")?;
    let barrel = intersection(&whole, &slab).map_err(e)?;
    shrunk(&barrel, 1.)
}
