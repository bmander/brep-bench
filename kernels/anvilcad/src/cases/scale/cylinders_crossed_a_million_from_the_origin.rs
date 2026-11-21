//! The common part of two cylinders of radius 3 and length 20 crossing square at
//! c = (600000, −800000, 0), along (2, 3, 6)/7 and (3, −6, 2)/7.
use anvil_brep::Solid;

use crate::shared::{intersect, rod};

pub fn build() -> Result<Solid, String> {
    let c = [600000., -800000., 0.];
    let along = |u: [f64; 3], s: f64| [c[0] + s * u[0] / 7., c[1] + s * u[1] / 7., c[2] + s * u[2] / 7.];
    let (u, v) = ([2., 3., 6.], [3., -6., 2.]);
    intersect(&rod(along(u, -10.), along(u, 10.), 3.)?, &rod(along(v, -10.), along(v, 10.), 3.)?)
}
