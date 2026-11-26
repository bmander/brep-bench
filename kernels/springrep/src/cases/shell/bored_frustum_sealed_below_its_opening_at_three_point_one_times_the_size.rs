//! `shell/bored-frustum-sealed-below-its-opening` with every length 3.1 times as long: a cone
//! frustum on the z axis, radius 18.6 at z = 0 to 9.3 at z = 12.4, bored through by a hole of
//! radius 3.1, shelled inward to a wall 3.1 thick, open at its top. The moved side meets the moved
//! bore below the top, so the cavity is a closed ring and the top stays. Built as the original is,
//! every length times 3.1.
use springrep::boolean::difference;
use springrep::edit::Wall;
use springrep::primitive::{cone, cylinder};
use springrep::topo::Brep;

use crate::shared::shell::shelled;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let frustum = cone(&at([0., 0., 0.]), 18.6, 9.3, 12.4, "frustum").map_err(e)?;
    let hole = cylinder(&at([0., 0., -3.1]), 3.1, 18.6, "bore").map_err(e)?;
    let bored = difference(&frustum, &hole).map_err(e)?;
    // the top, the annulus between radii 3.1 and 9.3, keeps its name
    shelled(&bored, &["frustum.top"], 3.1, Wall::Inward, "shell")
}
