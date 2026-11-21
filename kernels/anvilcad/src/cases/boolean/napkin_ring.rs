//! A napkin ring: the ball of radius 5 about the origin bored through along z by a hole of
//! radius 3.
use anvil_brep::{Frame, Solid};

use crate::shared::{ball, cylinder, subtract};

pub fn build() -> Result<Solid, String> {
    let whole = ball([0., 0., 0.], 5.)?;
    // the drill runs past the ball at both ends
    let drill = cylinder(&Frame::world(), 3., -6., 6.)?;
    subtract(&whole, &drill)
}
