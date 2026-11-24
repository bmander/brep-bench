//! A plate from the origin to (40, 30, 6) with all twelve edges rounded to radius 3, half its
//! thickness, in one operation: the side faces vanish, and the balls at the top and bottom of each
//! upright edge are one, so that edge keeps no round.
use springrep::blend::fillet_edges;
use springrep::math::Vec3;
use springrep::pick::convex_edges;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let plate = block(&at([0., 0., 0.]), Vec3::new(40., 30., 6.), "plate").map_err(e)?;
    let edges = convex_edges(&plate).map_err(e)?;
    fillet_edges(&plate, &edges, 3., "round").map_err(e)
}
