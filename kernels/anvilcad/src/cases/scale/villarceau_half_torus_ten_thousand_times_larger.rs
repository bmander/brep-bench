//! A torus about the z axis (middle circle 50000, tube 30000) cut by the bitangent plane
//! 3x − 4z = 0, keeping what lies below it.
use anvil_brep::Solid;

use crate::shared::{beyond, frame, subtract, torus};

pub fn build() -> Result<Solid, String> {
    // the tube's seam in the plane x = 0, on the side y > 0
    let ring = torus(&frame([0., 0., 0.], [0., 0., 1.], [0., 1., 0.])?, 50000., 30000.)?;
    // everything above the plane, its normal (−3, 0, 4) pointing up and away from the kept side
    subtract(&ring, &beyond([0., 0., 0.], [-3., 0., 4.], 200000.)?)
}
