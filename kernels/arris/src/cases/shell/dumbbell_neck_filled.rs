//! A dumbbell-shaped plate, two 10 × 10 squares joined by a neck 4 long and 2 wide, extruded 6,
//! shelled inward 1.5 thick with its top open: the neck fills, and each pocket bulges towards it in
//! arcs about the neck's corners.
//!
//! Refused. arris's shell joins the moved faces sharp at every edge, and its offset refuses walls
//! that cross; the wall asked for here is every point within 1.5 of the plate's faces, the neck's
//! walls crossing and the pockets bounded by arcs about the neck's concave corners, which no
//! fillet of a sharp shell makes, since there is no sharp shell to round.
use crate::Refusal;
use arris::topo::{Body, Model};

pub fn build(_: &mut Model) -> Result<Body, Refusal> {
    Err("arris's shell joins moved faces sharp and refuses walls that cross: it has no shell \
         joined by arcs, which is what trims the neck's crossing walls and bounds the pockets by \
         arcs about the neck's corners"
        .into())
}
