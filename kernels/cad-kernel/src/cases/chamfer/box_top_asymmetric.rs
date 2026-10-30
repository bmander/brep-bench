//! A box from the origin to (20, 10, 6), the four edges round its top chamfered 1 across the top
//! and 2 down the side.
use crate::sketch::polygon;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::prism;
use cad_kernel::fillet::flat_along;
use cad_kernel::geom::tolerance;
use cad_kernel::topo::Brep;

const LENGTH: f64 = 20.;
const WIDTH: f64 = 10.;
const HEIGHT: f64 = 6.;
const ACROSS_TOP: f64 = 1.;
const DOWN_SIDE: f64 = 2.;

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
    let mut part = prism(&floor, 0., HEIGHT)?;
    // the middle of the top face: on the top's plane and off every side's, it says which face of
    // each edge takes the first leg (1, measured from the side's plane) — the second, 2 from the
    // top's plane, is on the side
    let top = [LENGTH / 2., WIDTH / 2., HEIGHT];
    // the top's four edges, each picked by its middle, one after another: each flat after the first
    // is cut off where it reaches the ones before
    let edges = [
        [LENGTH / 2., 0., HEIGHT],
        [LENGTH, WIDTH / 2., HEIGHT],
        [LENGTH / 2., WIDTH, HEIGHT],
        [0., WIDTH / 2., HEIGHT],
    ];
    for at in edges {
        let legs = [ACROSS_TOP, DOWN_SIDE];
        let piece = flat_along(&part, at, legs, Some(top), tolerance(part.size()))?;
        part = combined(&part, &piece, Op::Cut, 0.)?;
    }
    Ok(part)
}
