//! `loft/square-to-round` three times larger: a transition from the square −60 ≤ x, y ≤ 60 at
//! z = 0 to the circle of radius 45 at z = 90: four flat triangles and four pieces of oblique
//! cones, the hull of the two. Built as the original is, every length times 3.
use springrep::loft::{Section, loft_developable};
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let sections = [
        Section::new(Profile::rect(Vec2::ZERO, 120., 120.), at([0., 0., 0.])),
        Section::new(Profile::circle(Vec2::ZERO, 90.), at([0., 0., 90.])),
    ];
    // each ruling joins points where the two outlines' tangents run the same way: a side of the
    // square meets the circle's nearest point, and a corner the quarter circle facing it
    loft_developable(&sections, "transition").map_err(e)
}
