//! A cylinder of radius 5 on the z axis from z = −1 to 10, kept above z = 0 and below the plane
//! z = x tan 30°: a hoof.
use anvil_brep::{Frame, Solid};

use crate::shared::{beyond, cut_all, cylinder};

pub fn build() -> Result<Solid, String> {
    let (s, c) = 30f64.to_radians().sin_cos();
    let cuts = [beyond([0., 0., 0.], [0., 0., -1.], 20.), beyond([0., 0., 0.], [-s, 0., c], 20.)];
    cut_all(cylinder(&Frame::world(), 5., -1., 10.)?, cuts)
}
