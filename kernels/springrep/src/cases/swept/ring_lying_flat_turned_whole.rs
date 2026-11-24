//! The solid a ring fills as it turns a whole turn about the z axis: the torus about the upright
//! line through (8, 0), its tube's centers on the circle of radius 3 in the plane z = 0, its tube
//! of radius 1.
//!
//! springrep's sweep of a solid declines this one (it cannot tell whether a face piece is inside).
//! A circle about the z axis that crosses the ring's hole meets the ring on both sides of it, so a
//! whole turn sweeps the hole, filled, too; the ring with its hole filled meets every circle about
//! the z axis in one arc, across the plane y = 0. So the sweep is that section, the stadium from ρ
//! = 4 to 12 of height 2, turned a whole turn (`shared::swept`, a turn).
use std::f64::consts::{PI, TAU};

use springrep::topo::Brep;

use crate::shared::swept::{arc, line, outline, swing};

pub fn build() -> Result<Brep, String> {
    let section = outline(vec![
        line((5.0, -1.0), (11.0, -1.0), "floor"),
        arc((11.0, 0.0), 1.0, -PI / 2.0, PI, "outer"),
        line((11.0, 1.0), (5.0, 1.0), "roof"),
        arc((5.0, 0.0), 1.0, PI / 2.0, PI, "inner"),
    ]);
    swing(&section, 0.0, TAU, "swept")
}
