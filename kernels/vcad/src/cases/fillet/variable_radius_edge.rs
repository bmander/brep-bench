//! A cube of side 10 from the origin with the edge from (10, 10, 0) to (10, 10, 10) rounded with a
//! radius growing linearly along it, from 1 at z = 0 to 2 at z = 10.
use vcad_kernel::vcad_kernel_fillet::{BlendKey, BlendSection, EdgeQuery};
use vcad_kernel::vcad_kernel_math::Point3;
use vcad_kernel::Solid;

const SIDE: f64 = 10.0;
const ROUND_BOTTOM: f64 = 1.0;
const ROUND_TOP: f64 = 2.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let cube = Solid::cube(SIDE, SIDE, SIDE);
    // the edge runs from `a` (t = 0) to `b` (t = 1); between the keys the radius grows linearly
    let edge = EdgeQuery::Endpoints {
        a: Point3::new(SIDE, SIDE, 0.0),
        b: Point3::new(SIDE, SIDE, SIDE),
    };
    let keys = [
        BlendKey { t: 0.0, section: BlendSection { size: ROUND_BOTTOM, shape: 1.0 } },
        BlendKey { t: 1.0, section: BlendSection { size: ROUND_TOP, shape: 1.0 } },
    ];
    Ok(cube.edge_blend(&edge, &keys)?)
}
