//! A block from the origin to (10, 10, 5) whose upright edge at x = 10, y = 10 is broken by a flat
//! 0.0001 back along each side: the prism from z = 0 to 5 on (0, 0), (10, 0), (10, 9.9999),
//! (9.9999, 10), (0, 10). The flat's two edges rounded together to radius 1, which swallows it.
//!
//! A ball of radius 1 has no room on a flat 0.00014 wide, so `rounds::rounded` reads the flat as
//! one the round swallows: it deletes it (`defeature::deleted`, the sides run on until they
//! meet), rounds the edge where they meet, and keeps the answer only where the flat is gone from it.
use crate::sketch::polygon;
use cad_kernel::build::prism;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::query::Located;
use cad_kernel::rounds::rounded;
use cad_kernel::topo::Brep;

const BREAK: f64 = 0.0001;
const HEIGHT: f64 = 5.;
const ROUND: f64 = 1.;

pub fn build() -> Result<Brep, String> {
    let (near, far) = ([10., 10. - BREAK, 0.], [10. - BREAK, 10., 0.]);
    // the floor, counter-clockwise from above so it faces up, swept up; its sides named, since a
    // round reads faces of one name as pieces of one face (the caps are `far` and `near`)
    let mut floor = polygon(&[[0., 0., 0.], [10., 0., 0.], near, far, [0., 10., 0.]]);
    floor.names = vec![["front", "right", "flat", "back", "left"].map(String::from).to_vec()];
    let block = prism(&floor, 0., HEIGHT)?;
    // the flat's two upright edges, each the edge passing through its corner's middle height
    let tol = tolerance(block.size());
    let located = Located::new(&block, tol);
    let through = |[x, y, _]: V| {
        let at = [x, y, HEIGHT / 2.];
        (0..block.edges.len() as u32)
            .find(|&e| located.edge_gap(e, at) <= tol && located.nearest(e, at).1 <= tol)
            .map(|e| e as usize)
            .ok_or_else(|| format!("no edge through {at:?}"))
    };
    let edges = [through(near)?, through(far)?];
    // both rounded in one call: the kernel finds the faces either side of each and rolls the ball
    rounded(&block, &edges, ROUND, tol)
}
