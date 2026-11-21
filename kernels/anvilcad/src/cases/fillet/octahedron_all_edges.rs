//! The octahedron |x| + |y| + |z| ≤ 10 with all twelve edges rounded to radius 2.
use anvil_brep::Solid;

use crate::shared::{fillet, octahedron};

pub fn build() -> Result<Solid, String> {
    // each edge's middle: halfway between two corners on different axes
    let mut edges = Vec::new();
    for s in [-5., 5.] {
        for t in [-5., 5.] {
            edges.extend([[s, t, 0.], [s, 0., t], [0., s, t]]);
        }
    }
    fillet(&octahedron(10.)?, &edges, 2.)
}
