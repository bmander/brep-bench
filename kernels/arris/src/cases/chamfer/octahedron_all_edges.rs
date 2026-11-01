//! The octahedron |x| + |y| + |z| ≤ 10 with all twelve of its edges chamfered √6 on both faces,
//! together, four chamfers meeting at each corner and the corner cut flat by a square.
use crate::solids::octahedron;
use crate::Refusal;
use arris::math::Control;
use arris::ops::{chamfer, OpError};
use arris::topo::{Body, Model};

const SIZE: f64 = 10.0;
/// √6, across a face square to the edge: the chamfer of the edge from (10, 0, 0) to (0, 10, 0)
/// meets the face x + y + z = 10 on the line z = 2.
const DISTANCE: f64 = 2.449489742783178;

pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    let solid = octahedron(m, SIZE)?;
    // every edge of the octahedron in one chamfer; arris 0.9.1 declines the corners, where four
    // chamfers meet: its closed forms close a corner of three ("the corner at the edge's end is
    // one the blend's closed forms do not cover")
    let edges = m.edges(solid).map_err(OpError::from)?;
    let (chamfered, _) = chamfer(m, solid, &edges, DISTANCE, &Control::NONE)?;
    Ok(chamfered)
}
