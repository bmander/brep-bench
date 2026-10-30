//! A block from the origin to (10, 4, 6), the two edges of its top running along x chamfered 2 on
//! both faces, half the top's width: the top vanishes into a ridge.
use crate::sketch::polygon;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::prism;
use cad_kernel::fillet::flat_along;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::topo::Brep;

const LENGTH: f64 = 10.;
const WIDTH: f64 = 4.;
const HEIGHT: f64 = 6.;
const DISTANCE: f64 = 2.;

/// `part` less the chamfer of its convex edge through `at`: the flat the kernel cuts along the
/// edge between a leg on each face, 2 from the other face, and the piece between it and the edge.
fn chamfered(part: &Brep, at: V) -> Result<Brep, String> {
    let piece = flat_along(part, at, [DISTANCE, DISTANCE], None, tolerance(part.size()))?;
    Ok(combined(part, &piece, Op::Cut, 0.)?)
}

pub fn build() -> Result<Brep, String> {
    // the floor, counter-clockwise from above so it faces up, swept up; its sides named, since a
    // chamfer reads faces of one name as pieces of one face (the caps are `far` and `near`)
    let mut floor = polygon(&[
        [0., 0., 0.],
        [LENGTH, 0., 0.],
        [LENGTH, WIDTH, 0.],
        [0., WIDTH, 0.],
    ]);
    floor.names = vec![["front", "right", "back", "left"].map(String::from).to_vec()];
    let slab = prism(&floor, 0., HEIGHT)?;
    // the top's edge at y = 0, then the one at y = 4, each picked by its middle
    let slab = chamfered(&slab, [LENGTH / 2., 0., HEIGHT])?;
    chamfered(&slab, [LENGTH / 2., WIDTH, HEIGHT])
}
