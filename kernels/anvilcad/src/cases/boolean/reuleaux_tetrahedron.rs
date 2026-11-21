//! The common part of four balls of radius 10 centered at the vertices of a regular tetrahedron
//! of side 10: a Reuleaux tetrahedron.
use anvil_brep::Solid;

use crate::shared::{ball, intersect};

pub fn build() -> Result<Solid, String> {
    let s3 = 3f64.sqrt();
    let centers = [[0., 0., 0.], [10., 0., 0.], [5., 5. * s3, 0.], [5., 5. * s3 / 3., 10. * (2f64 / 3.).sqrt()]];
    let mut body = ball(centers[0], 10.)?;
    for c in &centers[1..] {
        body = intersect(&body, &ball(*c, 10.)?)?;
    }
    Ok(body)
}
