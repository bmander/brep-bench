//! A cube of side 10 from the origin with all twelve edges chamfered 2 on both faces, one after
//! another, each ending on the faces at its ends: the corners mitered.
use anvil_brep::Solid;

use crate::shared::{block, chamfer};

pub fn build() -> Result<Solid, String> {
    let mut edges = Vec::new();
    for a in [0., 10.] {
        for b in [0., 10.] {
            edges.extend([[5., a, b], [a, 5., b], [a, b, 5.]]);
        }
    }
    chamfer(&block([0., 0., 0.], [10., 10., 10.])?, &edges, 2.)
}
