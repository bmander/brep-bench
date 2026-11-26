//! The L-shaped plate of `offset/l-block-grown` with its inside corner's edge rounded to radius 4,
//! grown by 5, past the round's radius: springrep's own fillet, then its own offset.
use springrep::blend::fillet;
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::offset::grown;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let outline = [(0., 0.), (20., 0.), (20., 5.), (5., 5.), (5., 15.), (0., 15.)]
        .map(|(x, y)| Vec2::new(x, y));
    // the inside corner is where the ledge (y = 5) meets the riser (x = 5)
    let section = Profile::polygon(&outline, &["front", "end", "ledge", "riser", "top", "back"]);
    let plate = extrude(&section, &at([0., 0., 0.]), 8., "plate").map_err(e)?;
    let rounded = fillet(&plate, &[("plate.ledge", "plate.riser")], 4., "round").map_err(e)?;
    grown(&rounded, 5.)
}
