//! A ball of radius 10 with 62 dimples, each cut by a ball of radius 1.2 centered 10.6 from the
//! origin toward a vertex, an edge's midpoint or a face's center of the icosahedron with vertices
//! (0, ±1, ±φ), (±1, ±φ, 0), (±φ, 0, ±1).
use anvil_brep::Solid;
use glam::DVec3;

use crate::shared::{ball, cut_all};

pub fn build() -> Result<Solid, String> {
    let phi = (1. + 5f64.sqrt()) / 2.;
    let mut v = Vec::new();
    for s in [-1., 1.] {
        for t in [-1., 1.] {
            v.push(DVec3::new(0., s, t * phi));
            v.push(DVec3::new(s, t * phi, 0.));
            v.push(DVec3::new(t * phi, 0., s));
        }
    }
    // the edges join vertices 2 apart, and the faces are the triangles of three such
    let edge = |a: DVec3, b: DVec3| ((a - b).length() - 2.).abs() < 1e-9;
    let mut dirs: Vec<DVec3> = v.clone();
    for i in 0..12 {
        for j in i + 1..12 {
            if edge(v[i], v[j]) {
                dirs.push(v[i] + v[j]);
                for k in j + 1..12 {
                    if edge(v[i], v[k]) && edge(v[j], v[k]) {
                        dirs.push(v[i] + v[j] + v[k]);
                    }
                }
            }
        }
    }
    if dirs.len() != 62 {
        return Err(format!("{} directions, not 62", dirs.len()));
    }
    // each small ball's poles lie on the vertical through its center (its revolution's axis is z)
    let dimples = dirs.into_iter().map(|d| ball((d.normalize() * 10.6).to_array(), 1.2));
    cut_all(ball([0., 0., 0.], 10.)?, dimples)
}
