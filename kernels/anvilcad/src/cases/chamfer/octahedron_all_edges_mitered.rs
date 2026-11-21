//! The octahedron |x| + |y| + |z| ≤ 10 with all twelve edges chamfered √6 on both faces, one after
//! another, each ending on the faces at its ends: the corners mitered.
use anvil_brep::Solid;

use crate::shared::{chamfer, octahedron};

pub fn build() -> Result<Solid, String> {
    let mut edges = Vec::new();
    for s in [-5., 5.] {
        for t in [-5., 5.] {
            edges.extend([[s, t, 0.], [s, 0., t], [0., s, t]]);
        }
    }
    let chamfered = chamfer(&octahedron(10.)?, &edges, 6f64.sqrt())?;
    Ok(chamfered)
}
