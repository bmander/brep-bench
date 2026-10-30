//! A plate from the origin to (20, 10, 2) with the two edges of its face y = 10 that run along x
//! rounded to radius 0.9999, leaving a strip of that face 0.0002 tall between the rounds.
use vcad_kernel::vcad_kernel_fillet::{BlendKey, BlendSection, EdgeQuery};
use vcad_kernel::vcad_kernel_math::Point3;
use vcad_kernel::Solid;

const LENGTH: f64 = 20.0;
const WIDTH: f64 = 10.0;
const THICKNESS: f64 = 2.0;
const ROUND: f64 = 0.9999;

pub fn build() -> Result<Solid, crate::Refusal> {
    let mut plate = Solid::cube(LENGTH, WIDTH, THICKNESS);
    let round = BlendKey { t: 0.0, section: BlendSection { size: ROUND, shape: 1.0 } };
    // the face y = 10's edges along x, at z = 0 and z = 2, each picked by its two ends
    for z in [0.0, THICKNESS] {
        let edge = EdgeQuery::Endpoints {
            a: Point3::new(0.0, WIDTH, z),
            b: Point3::new(LENGTH, WIDTH, z),
        };
        plate = plate.edge_blend(&edge, &[round])?;
    }
    Ok(plate)
}
