//! `swept/box-screwed-inside-a-rod` a hundred times larger: the rod of radius 500 about the z axis
//! from z = 0 to z = 2000 less everything the box 250 ≤ x ≤ 350, |y| ≤ 50, 200 ≤ z ≤ 300 passes
//! through as it is screwed about the z axis, rising 400 a turn, from a turn of 0° to one of 720°:
//! a sealed helical tunnel. Built as the original is, every length times 100 (the shared rod
//! written out here at the larger size).
use springrep::boolean::difference;
use springrep::primitive::cylinder;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::{at, e};
use crate::shared::swept::{boxed, deg, screwing};

pub fn build() -> Result<Brep, String> {
    let bar = boxed([250.0, -50.0, 200.0], [350.0, 50.0, 300.0], "bar")?;
    let swept = solid(&bar, &screwing(400.0, 0.0, deg(720.0)), "screw").map_err(e)?;
    let rod = cylinder(&at([0.0, 0.0, 0.0]), 500.0, 2000.0, "rod").map_err(e)?;
    difference(&rod, &swept).map_err(e)
}
