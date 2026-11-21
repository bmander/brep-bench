//! A tee: the cylinder of radius 6 about the x axis from x = −20 to 20, and the cylinder of radius
//! 3 about the z axis from z = 0 to 15 standing out of it; the saddle where they meet rounded to
//! radius 1.
use anvil_brep::{Frame, Solid};

use crate::shared::{cylinder, fillet, rod, union};

pub fn build() -> Result<Solid, String> {
    let tee = union(&rod([-20., 0., 0.], [20., 0., 0.], 6.)?, &cylinder(&Frame::world(), 3., 0., 15.)?)?;
    // the junction, picked where it passes over y = 3, at z = √27
    fillet(&tee, &[[0., 3., 27f64.sqrt()]], 1.)
}
