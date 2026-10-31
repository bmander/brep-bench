//! A square column from z = 0 to 40 whose section at height z is the square of side 30 − z/2
//! centred on the z axis, turned right-handed about it by 4.5° a unit of height: its corners on
//! four conical spirals, its walls the segments between them; flat ends.
use cad_kernel::family::{family, Run, Sheet, Stations, Wall, NODES};
use cad_kernel::geom::V;
use cad_kernel::topo::Brep;
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, SQRT_2};
use std::sync::Arc;

const HEIGHT: f64 = 40.;
/// The turn a unit of height, radians (4.5°).
const TWIST: f64 = std::f64::consts::PI / 40.;
/// How many pieces the height is read in, six stations each: between them each corner is the
/// degree-five polynomial through its places, which misses its spiral over a piece of 1.25 high
/// (a turn of 5.6°) by under 10⁻¹² of its size.
const PIECES: usize = 32;

/// The corner `k` of the section at height `z`, counter-clockwise from the one that starts at
/// (15, 15, 0): (15 − z/4)√2 from the z axis, at 45° + 90°k + 4.5°z from +x.
fn corner(k: usize, z: f64) -> V {
    let r = (15. - z / 4.) * SQRT_2;
    let (s, c) = (FRAC_PI_4 + FRAC_PI_2 * k as f64 + TWIST * z).sin_cos();
    [r * c, r * s, z]
}

pub fn build() -> Result<Brep, String> {
    // a loft through a section that changes as it rises is a face family over the height, in
    // pieces each with its six stations at the Gauss–Lobatto nodes
    let step = HEIGHT / PIECES as f64;
    let spans: Vec<[f64; 2]> =
        (0..PIECES).map(|j| [step * j as f64, step * (j + 1) as f64]).collect();
    let at = spans.iter().map(|&[a, b]| NODES.map(|t| (a + b) / 2. + t * (b - a) / 2.)).collect();
    let stations = Arc::new(Stations::new(spans, at)?);
    // each side a straight segment between neighbouring spirals: a twisted ruled wall
    let walls = (0..4).map(|k| {
        let nodes = stations.at.iter().flatten().map(|&z| {
            let (a, b) = (corner(k, z), corner((k + 1) % 4, z));
            vec![a[0], a[1], a[2], b[0], b[1], b[2]]
        }).collect();
        let sheet = Sheet::new(Run::Line, stations.clone(), nodes, false)?;
        Ok(Wall { sheet: Arc::new(sheet), name: "side".into() })
    }).collect::<Result<Vec<_>, String>>()?;
    family(&[walls], 0., None)
}
