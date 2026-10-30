//! A cube of side 10 from the origin with the edge where its faces x = 10 and y = 10 meet rounded
//! to radius 2.
use vcad_kernel::vcad_kernel_fillet::{BlendKey, BlendSection, EdgeQuery};
use vcad_kernel::vcad_kernel_math::Point3;
use vcad_kernel::Solid;

const SIDE: f64 = 10.0;
const ROUND: f64 = 2.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let cube = Solid::cube(SIDE, SIDE, SIDE);
    // the edge picked by its two ends; shape 1 is a round, its size the radius
    let edge = EdgeQuery::Endpoints {
        a: Point3::new(SIDE, SIDE, 0.0),
        b: Point3::new(SIDE, SIDE, SIDE),
    };
    let round = BlendKey { t: 0.0, section: BlendSection { size: ROUND, shape: 1.0 } };
    Ok(cube.edge_blend(&edge, &[round])?)
}
