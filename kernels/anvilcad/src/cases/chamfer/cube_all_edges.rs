//! A cube of side 10 from the origin with all twelve edges chamfered 2 on both faces, and each
//! corner cut flat by the plane through the three square corners nearest it.
use anvil_brep::Solid;

use crate::shared::{beyond, block, chamfer, cut_all};

pub fn build() -> Result<Solid, String> {
    let mut edges = Vec::new();
    for a in [0., 10.] {
        for b in [0., 10.] {
            edges.extend([[5., a, b], [a, 5., b], [a, b, 5.]]);
        }
    }
    let chamfered = chamfer(&block([0., 0., 0.], [10., 10., 10.])?, &edges, 2.)?;
    // the corner planes: at (10, 10, 10) the plane x + y + z = 26, the others likewise
    let mut corners = Vec::new();
    for sx in [-1., 1.] {
        for sy in [-1., 1.] {
            for sz in [-1., 1.] {
                // the point of the plane nearest the cube's center, 11/√3 from it
                let p = [5. + sx * 11. / 3., 5. + sy * 11. / 3., 5. + sz * 11. / 3.];
                corners.push(beyond(p, [sx, sy, sz], 5.));
            }
        }
    }
    cut_all(chamfered, corners)
}
