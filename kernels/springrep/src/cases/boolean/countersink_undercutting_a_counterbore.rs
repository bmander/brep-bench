//! A disc of radius 100 from z = −20 to 20, counterbored to radius 60 down to z = 0, and drilled
//! 15 deep from the counterbore's floor by a hole of radius 3.5 about the upright through
//! (59.9, 0), 0.1 inside the wall, its mouth countersunk at 90° to radius 4: the hole and its
//! countersink reach under the wall.
use springrep::boolean::difference;
use springrep::primitive::{cone, cylinder};
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let disc = cylinder(&at([0., 0., -20.]), 100., 40., "disc").map_err(e)?;
    // the counterbore runs out past the disc's top
    let counterbore = cylinder(&at([0., 0., 0.]), 60., 30., "counterbore").map_err(e)?;
    let body = difference(&disc, &counterbore).map_err(e)?;
    // the hole from its flat bottom up to the floor, its top flush with it
    let hole = cylinder(&at([59.9, 0., -15.]), 3.5, 15., "hole").map_err(e)?;
    let body = difference(&body, &hole).map_err(e)?;
    // the countersink: the 90° cone from its point 4 below the floor to the circle of radius 4 on it
    let countersink = cone(&at([59.9, 0., -4.]), 0., 4., 4., "countersink").map_err(e)?;
    difference(&body, &countersink).map_err(e)
}
