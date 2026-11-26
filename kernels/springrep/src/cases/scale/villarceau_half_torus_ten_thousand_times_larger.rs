//! `tangent/villarceau-half-torus` ten thousand times larger: the torus about z (middle radius
//! 50000, tube radius 30000) cut by its bitangent plane 3x - 4z = 0, keeping what lies on and below
//! it: the cut face is two crescents between Villarceau's circles. Built as the original is, every
//! length times 10000.
use springrep::boolean::intersection;
use springrep::primitive::torus;
use springrep::topo::Brep;

use crate::shared::tangent::half_space;
use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // the seam in the plane x = 0 on the side y > 0, away from the two tangent points
    let ring = torus(&frame([0., 0., 0.], [0., 0., 1.], [0., 1., 0.])?, 50000., 30000., "torus")
        .map_err(e)?;
    // the half-space z ≤ 3x/4, on the side of the plane its normal (3, 0, -4) points to
    let below = half_space([0., 0., 0.], [3., 0., -4.], 200000., "below")?;
    intersection(&ring, &below).map_err(e)
}
