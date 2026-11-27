//! The ball of radius 10 about the origin less the ball of radius 0.8 about (ρ cos 120°,
//! ρ sin 120°, 5.501), ρ = √(10.4² − 5.501²): one dimple, its meeting circle a hair from the small
//! ball's lower pole.
use anvil_brep::Solid;

use crate::shared::{ball, subtract};

pub fn build() -> Result<Solid, String> {
    let (h, rho) = (5.501f64, (10.4f64 * 10.4 - 5.501 * 5.501).sqrt());
    let a = 120f64.to_radians();
    // each revolved about the vertical through its center, so its poles lie on that vertical
    subtract(&ball([0., 0., 0.], 10.)?, &ball([rho * a.cos(), rho * a.sin(), h], 0.8)?)
}
