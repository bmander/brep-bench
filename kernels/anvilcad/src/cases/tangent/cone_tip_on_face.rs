//! A block from (−10, −10, 0) to (10, 10, 10) united with a cone on the z axis whose apex is the
//! origin and whose base is the disk of radius 5 at z = 15.
use anvil_brep::{Frame, Solid};

use crate::shared::{block, cone, union};

pub fn build() -> Result<Solid, String> {
    union(&block([-10., -10., 0.], [10., 10., 10.])?, &cone(&Frame::world(), 0., 5., 15.)?)
}
