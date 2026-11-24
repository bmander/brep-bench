//! A bar (−40, −8, −5)–(40, 8, 5) joined to an eye, the cylinder of radius 14 about the y axis
//! from y = −8 to 8, the four concave edges where the eye's side meets the bar's top and bottom
//! rounded to radius 3, filling.
use springrep::blend::fillet;
use springrep::boolean::union;
use springrep::math::Vec3;
use springrep::primitive::{block, cylinder};
use springrep::topo::Brep;

use crate::shared::{at, e, frame};

pub fn build() -> Result<Brep, String> {
    let bar = block(&at([-40., -8., -5.]), Vec3::new(80., 16., 10.), "bar").map_err(e)?;
    let along_y = frame([0., -8., 0.], [0., 1., 0.], [1., 0., 0.])?;
    let eye = cylinder(&along_y, 14., 16., "eye").map_err(e)?;
    let body = union(&bar, &eye).map_err(e)?;
    // the bar's top is its end, its bottom its start
    fillet(&body, &[("eye.side", "bar.end"), ("eye.side", "bar.start")], 3., "round").map_err(e)
}
