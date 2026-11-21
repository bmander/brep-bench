//! A turned post on the z axis, the cylinder of radius 5 from z = 0 to 6 topped by the frustum from
//! radius 5 at z = 6 to 2 at z = 10, the edge between them rounded to radius 1; shrunk by 1.5.
use anvil_brep::Solid;

use crate::shared::{fillet, offset, polygon, ring, turned};

pub fn build() -> Result<Solid, String> {
    let post = turned(&anvil_brep::Frame::world(), polygon(&[(0., 0.), (5., 0.), (5., 6.), (2., 10.), (0., 10.)]))?;
    offset(&fillet(&post, &[ring(5., 6.)], 1.)?, -1.5)
}
