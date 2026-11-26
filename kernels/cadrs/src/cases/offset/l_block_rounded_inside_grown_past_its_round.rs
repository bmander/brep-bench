//! The L-shaped plate of `offset/l-block-grown` (the polygon through (0, 0), (20, 0), (20, 5),
//! (5, 5), (5, 15) and (0, 15), extruded 8 along +z) with its inside corner's concave edge, along
//! (5, 5), rounded to radius 4, grown by 5, more than the round's radius: the solid of every
//! point within 5 of it.
//!
//! The outline extruded, the inside edge picked by a point on it and rounded by cadrs's Fillet,
//! then cadrs's offset with rounded edges.
use crate::bench::{extrude, p3};
use crate::shared::fillet::fillet;
use crate::shared::offset::grown;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Pick, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let outline =
        Loop::polygon(&[[0.0, 0.0], [20.0, 0.0], [20.0, 5.0], [5.0, 5.0], [5.0, 15.0], [0.0, 15.0]]);
    let plate = extrude(k, &Profile::of(Plane::xy(0.0), vec![outline]), 8.0)?;
    let inside = k.edge_at(plate, p3(5.0, 5.0, 4.0))?;
    let rounded = fillet(k, plate, &[inside], 4.0)?;
    grown(k, rounded, 5.0)
}
