//! A plate from (−20, −20, 0) to (20, 20, 5) with a boss of radius 8 on the z axis from z = 5 to
//! 17, the boss's side drafted 4° about the plate's top, pulling along +z.
use anvil_brep::{Frame, Solid};

use crate::shared::{block, cylinder, draft, ring, union};

pub fn build() -> Result<Solid, String> {
    let body = union(&block([-20., -20., 0.], [20., 20., 5.])?, &cylinder(&Frame::world(), 8., 5., 17.)?)?;
    draft(&body, &[ring(8., 11.)], [0., 0., 5.], [0., 0., 1.], 4.)
}
