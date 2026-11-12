//! The L outline through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15) extruded 8 along +z,
//! its inside corner, the edge x = 5, y = 5, rounded to radius 2; then the round deleted, the faces
//! x = 5 and y = 5 run back to meet in that edge: the L-block with a sharp inside corner.
//!
//! The outline is extruded and the inside edge filleted, radius 2; then Delete face takes the
//! round.
use crate::bench::{extrude, p3};
use crate::shared::edit::deleted;
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Pick, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let corners = [[0.0, 0.0], [20.0, 0.0], [20.0, 5.0], [5.0, 5.0], [5.0, 15.0], [0.0, 15.0]];
    let outline = Loop::polygon(&corners);
    let b = extrude(k, &Profile::of(Plane::xy(0.0), vec![outline]), 8.0)?;
    let inside = k.edge_at(b, p3(5.0, 5.0, 4.0))?;
    let rounded = fillet(k, b, &[inside], 2.0)?;
    // the round at its middle: 2 from its axis, the line x = 7, y = 7, at 45°
    let s = 2f64.sqrt();
    let round = k.faces_at(rounded, &[p3(7.0 - s, 7.0 - s, 4.0)])?;
    deleted(k, rounded, &round)
}
