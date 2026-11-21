//! The octahedron |x| + |y| + |z| ≤ 10 with all twelve edges chamfered √6 on both faces, and each
//! corner cut flat by the square through the four triangle corners nearest it.
use anvil_brep::Solid;

use crate::shared::{beyond, cut_all, chamfer, octahedron};

pub fn build() -> Result<Solid, String> {
    let mut edges = Vec::new();
    for s in [-5., 5.] {
        for t in [-5., 5.] {
            edges.extend([[s, t, 0.], [s, 0., t], [0., s, t]]);
        }
    }
    let chamfered = chamfer(&octahedron(10.)?, &edges, 6f64.sqrt())?;
    // the corner planes |x| = 6, |y| = 6 and |z| = 6
    let corners = [[1., 0., 0.], [-1., 0., 0.], [0., 1., 0.], [0., -1., 0.], [0., 0., 1.], [0., 0., -1.]]
        .map(|n: [f64; 3]| beyond(n.map(|c| 6. * c), n, 5.));
    cut_all(chamfered, corners)
}
