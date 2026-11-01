//! A cube of side 10 at the origin with all twelve edges chamfered 2 on both faces, one after
//! another, so the three chamfers at each corner mitre, meeting in a point.
use crate::sketch::polygon;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::prism;
use cad_kernel::fillet::flat_along;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::topo::Brep;

const SIDE: f64 = 10.;
const DISTANCE: f64 = 2.;

/// `part` less the chamfer of its edge through `at`: the flat the kernel cuts along the edge
/// between a leg on each face, 2 from the other face (between square faces, 2 across the face),
/// cut off at each end by the faces there, the chamfers made before it among them.
fn chamfered(part: &Brep, at: V) -> Result<Brep, String> {
    let piece = flat_along(part, at, [DISTANCE, DISTANCE], None, tolerance(part.size()))?;
    Ok(combined(part, &piece, Op::Cut, 0.)?)
}

pub fn build() -> Result<Brep, String> {
    // the floor, counter-clockwise from above so it faces up, swept up; its sides named, since a
    // chamfer reads faces of one name as pieces of one face (the caps are `far` and `near`)
    let mut floor = polygon(&[[0., 0., 0.], [SIDE, 0., 0.], [SIDE, SIDE, 0.], [0., SIDE, 0.]]);
    floor.names = vec![["front", "right", "back", "left"].map(String::from).to_vec()];
    let mut cube = prism(&floor, 0., SIDE)?;
    // each edge picked by its middle, which what is left of it keeps (every chamfer before it cuts
    // as much off either end): the four upright edges, then the four round the top, then the four
    // round the floor
    let half = SIDE / 2.;
    let mut middles: Vec<V> = Vec::new();
    for (x, y) in [(0., 0.), (SIDE, 0.), (SIDE, SIDE), (0., SIDE)] {
        middles.push([x, y, half]);
    }
    for z in [SIDE, 0.] {
        middles.extend([[half, 0., z], [SIDE, half, z], [half, SIDE, z], [0., half, z]]);
    }
    for at in middles {
        cube = chamfered(&cube, at)?;
    }
    Ok(cube)
}
