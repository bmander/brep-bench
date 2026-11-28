//! `tangent/villarceau-half-torus` ten thousand times larger, built as it is at every length: the
//! torus of radii 50000 and 30000 about the z axis, less the block above the bitangent plane
//! 3x − 4z = 0, reaching 200000 past the origin.
use crate::cases::tangent::villarceau_half_torus::halved;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    halved(10000.0)
}
