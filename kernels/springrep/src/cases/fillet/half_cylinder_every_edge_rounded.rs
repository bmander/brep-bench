//! The half of the cylinder of radius 5 from z = 0 to 10 where y ≥ 0, every edge rounded to radius
//! 1 in one operation: three rounds meet at each corner, closed by a ball.
use springrep::blend::fillet_edges;
use springrep::boolean::split;
use springrep::math::Vec3;
use springrep::pick::convex_edges;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let drum = cylinder(&at([0., 0., 0.]), 5., 10., "drum").map_err(e)?;
    // the half on the side y ≥ 0, the side the plane's normal points to
    let (half, _) = split(&drum, Vec3::ZERO, Vec3::Y).map_err(e)?;
    let edges = convex_edges(&half).map_err(e)?;
    fillet_edges(&half, &edges, 1., "round").map_err(e)
}
