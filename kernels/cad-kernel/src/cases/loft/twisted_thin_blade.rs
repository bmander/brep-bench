//! A blade from z = 0 to 60 whose section at height z is the rectangle 40 × 0.4 centred on the
//! z axis, turned right-handed about it by 6° a unit of height: a whole turn, its corners on four
//! helices, its walls the segments between them; flat ends.
use cad_kernel::family::{family, Run, Sheet, Stations, Wall, NODES};
use cad_kernel::geom::{Carry, V};
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;
use std::sync::Arc;

const HEIGHT: f64 = 60.;
/// The rectangle at the foot, counter-clockwise about z.
const FOOT: [V; 4] = [[-20., -0.2, 0.], [20., -0.2, 0.], [20., 0.2, 0.], [-20., 0.2, 0.]];
/// How many pieces the height is read in, six stations each: between them each corner is the
/// degree-five polynomial through its places, which misses its helix over a piece of 1.25 high
/// (a turn of 7.5°) by under 3·10⁻¹² of the blade's width — close enough for the kernel to hold
/// each wall to the exact surface the screw sweeps, and put it there.
const PIECES: usize = 48;

pub fn build() -> Result<Brep, String> {
    // the rectangle carried up by a screw: a whole turn right-handed about the z axis while it
    // rises 60
    let screw = Carry::Screw { center: [0.; 3], axis: [0., 0., 1.], angle: TAU, advance: HEIGHT };
    // a loft through a section that moves as it rises is a face family over the height, in pieces
    // each with its six stations at the Gauss–Lobatto nodes
    let step = HEIGHT / PIECES as f64;
    let spans: Vec<[f64; 2]> =
        (0..PIECES).map(|j| [step * j as f64, step * (j + 1) as f64]).collect();
    let at = spans.iter().map(|&[a, b]| NODES.map(|t| (a + b) / 2. + t * (b - a) / 2.)).collect();
    let stations = Arc::new(Stations::new(spans, at)?);
    // each side a straight segment between neighbouring helices, its ends the rectangle's corners
    // carried to the height
    let walls = (0..4).map(|k| {
        let nodes = stations.at.iter().flatten().map(|&z| {
            let v = z / HEIGHT;
            let (a, b) = (screw.at(FOOT[k], v), screw.at(FOOT[(k + 1) % 4], v));
            vec![a[0], a[1], a[2], b[0], b[1], b[2]]
        }).collect();
        let sheet = Sheet::new(Run::Line, stations.clone(), nodes, false)?;
        Ok(Wall { sheet: Arc::new(sheet), name: "side".into() })
    }).collect::<Result<Vec<_>, String>>()?;
    // the section carried rigidly, as stated: every wall lies exactly on what its foot's side
    // sweeps under the screw
    family(&[walls], 0., Some(screw))
}
