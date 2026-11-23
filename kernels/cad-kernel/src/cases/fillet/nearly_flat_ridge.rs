//! A prism from z = 0 to 6 on the pentagon (0, 0), (8, 0), (8, 5), (4, 5 + 4 tan 0.5°), (0, 5): a
//! roof of two faces each tilted 0.5° up to a ridge along x = 4, meeting at 179°. The ridge rounded
//! to radius 180.
//!
//! Refused, by the kernel: "`right roof` with `left roof` meet too nearly flat or too sharply to
//! round". `fillet::roll` rolls a ball only between faces turning more than 1° from one another,
//! (`fillet`'s `MIN_TURN`, π/180), and the two roof faces turn through 1°, at that bound.
//! `rounds::rounded` rolls the edge the same way: its one-call surgery between planes rounds only
//! a set of edges taking in every edge at each vertex it reaches, and the ridge's ends meet the
//! caps.
use crate::sketch::polygon;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::prism;
use cad_kernel::fillet::roll;
use cad_kernel::geom::tolerance;
use cad_kernel::topo::Brep;

const HEIGHT: f64 = 6.;
const ROUND: f64 = 180.;

pub fn build() -> Result<Brep, String> {
    let ridge = 5. + 4. * 0.5_f64.to_radians().tan();
    // the floor, counter-clockwise from above so it faces up, swept up; its sides named, since a
    // round reads faces of one name as pieces of one face (the caps are `far` and `near`)
    let mut floor = polygon(&[[0., 0., 0.], [8., 0., 0.], [8., 5., 0.], [4., ridge, 0.], [0., 5., 0.]]);
    floor.names = vec![["front", "right", "right roof", "left roof", "left"].map(String::from).to_vec()];
    let block = prism(&floor, 0., HEIGHT)?;
    // the ridge is picked by its middle; the ball rolls between the two roof faces
    let round = roll(&block, [4., ridge, HEIGHT / 2.], ROUND, tolerance(block.size()))?;
    // a convex edge's round takes its piece away
    let op = if round.concave { Op::Union } else { Op::Cut };
    Ok(combined(&block, &round.piece, op, 0.)?)
}
