//! A plate (−50, −50, 0)–(50, 50, 20) joined to a post, the cylinder of radius 15 about the
//! upright through (45, 0) from z = 0 to 40, which overhangs the plate's side; a hole of radius 6
//! drilled through along the post's axis, and both its rims chamfered 1.5.
use springrep::blend::chamfer;
use springrep::boolean::{difference, union};
use springrep::primitive::{block, cylinder};
use springrep::math::Vec3;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let plate = block(&at([-50., -50., 0.]), Vec3::new(100., 100., 20.), "plate").map_err(e)?;
    let post = cylinder(&at([45., 0., 0.]), 15., 40., "post").map_err(e)?;
    let body = union(&plate, &post).map_err(e)?;
    // the drill runs out past both faces
    let drill = cylinder(&at([45., 0., -1.]), 6., 42., "hole").map_err(e)?;
    let body = difference(&body, &drill).map_err(e)?;
    // the rim on the post's top, and the one on the bottom, which the plate's bottom and the
    // post's foot share
    chamfer(
        &body,
        &[("post.end", "hole.side"), ("plate.start", "hole.side"), ("post.start", "hole.side")],
        1.5,
        "bevel",
    )
    .map_err(e)
}
