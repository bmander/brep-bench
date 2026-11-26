//! The ball of radius 10 about the origin less the ball of radius 0.8 about (ρ cos 120°, ρ sin 120°,
//! 5.501), ρ = √(10.4² − 5.501²), poles upright: the circle where they meet passes a hair from the
//! small ball's lower pole.
use springrep::boolean::difference;
use springrep::primitive::sphere;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let height = 5.501;
    let rho = (10.4f64 * 10.4 - height * height).sqrt();
    let a = 120f64.to_radians();
    let ball = sphere(&at([0., 0., 0.]), 10., "ball").map_err(e)?;
    let cutter = sphere(&at([rho * a.cos(), rho * a.sin(), height]), 0.8, "dimple").map_err(e)?;
    difference(&ball, &cutter).map_err(e)
}
