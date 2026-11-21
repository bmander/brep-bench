//! The half y ≥ 0 of the torus about the z axis (middle circle 6, tube 2), shelled inward 0.5 thick, open at both flat faces.
use anvil_brep::Solid;

use crate::shared::{beyond, shell_open, subtract, torus};

pub fn build() -> Result<Vec<Solid>, String> {
    let t = subtract(&torus(&anvil_brep::Frame::world(), 6., 2.)?, &beyond([0., 0., 0.], [0., -1., 0.], 10.)?)?;
    shell_open(&t, &[[6., 0., 1.], [-6., 0., 1.]], 0.5)
}
