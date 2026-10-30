//! A block from the origin to (10, 4, 6) with the two edges of its top face that run along x
//! rounded to radius 2, half the top's width, so the top becomes a half-cylinder.
use vcad_kernel::vcad_kernel_fillet::{BlendKey, BlendSection, EdgeQuery};
use vcad_kernel::vcad_kernel_math::Point3;
use vcad_kernel::Solid;

const LENGTH: f64 = 10.0;
const WIDTH: f64 = 4.0;
const HEIGHT: f64 = 6.0;
const ROUND: f64 = 2.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let mut block = Solid::cube(LENGTH, WIDTH, HEIGHT);
    let round = BlendKey { t: 0.0, section: BlendSection { size: ROUND, shape: 1.0 } };
    // the top's edges along x, at y = 0 and y = 4, each picked by its two ends
    for y in [0.0, WIDTH] {
        let edge = EdgeQuery::Endpoints {
            a: Point3::new(0.0, y, HEIGHT),
            b: Point3::new(LENGTH, y, HEIGHT),
        };
        block = block.edge_blend(&edge, &[round])?;
    }
    Ok(block)
}
