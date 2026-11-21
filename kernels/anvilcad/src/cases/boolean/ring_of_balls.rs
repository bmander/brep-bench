//! Twelve balls of radius 3 centered 30° apart on the circle of radius 10 about the z axis, the
//! first at (10, 0, 0), joined one after another into a ring.
use anvil_brep::Solid;

use crate::shared::{ball, unite_all};

pub fn build() -> Result<Solid, String> {
    let at = |k: usize| {
        let a = (30. * k as f64).to_radians();
        ball([10. * a.cos(), 10. * a.sin(), 0.], 3.)
    };
    unite_all(at(0)?, (1..12).map(at))
}
