//! Two cylinders of radius 5 on the z axis, from z = 0 to 10 and from 6 to 16, united.
use anvil_brep::{Frame, Solid};

use crate::shared::{cylinder, frame, union};

pub fn build() -> Result<Solid, String> {
    // the second turned a quarter turn, so that the seams of the coincident walls differ
    let turned = frame([0., 0., 0.], [0., 0., 1.], [0., 1., 0.])?;
    union(&cylinder(&Frame::world(), 5., 0., 10.)?, &cylinder(&turned, 5., 6., 16.)?)
}
