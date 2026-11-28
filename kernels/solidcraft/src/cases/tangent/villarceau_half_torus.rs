//! A torus about the z axis, of radii 5 (its tube's middle circle) and 3 (its tube), cut by the
//! bitangent plane 3x - 4z = 0, keeping what lies on and below it, z <= 3x/4: the half-space
//! above is a block reaching past the torus, its bottom face first on the xy plane and then
//! tilted about the y axis until its normal is (-3, 0, 4)/5.
//!
//! solidcore's torus has its seam (the tube's section it was turned from) on its +x side, so it
//! is turned a quarter turn about z first, putting the seam in the plane x = 0 on the side y > 0.
use crate::bench::{cut, rotated};
use crate::Refusal;
use solidcore_ops::{make_box, make_torus, Body};

pub fn build() -> Result<Body, Refusal> {
    halved(1.0)
}

/// The torus and the block with every length `s` times as long.
pub fn halved(s: f64) -> Result<Body, Refusal> {
    let torus = make_torus([0.0, 0.0, 0.0], 5.0 * s, 3.0 * s)?;
    let torus = rotated(torus, [0.0; 3], [0.0, 0.0, 1.0], 90.0)?;
    let above = make_box([-20.0 * s, -20.0 * s, 0.0], [40.0 * s, 40.0 * s, 20.0 * s])?;
    // turning +z about y by -asin(3/5) takes it to (-3, 0, 4)/5
    let above = rotated(above, [0.0; 3], [0.0, 1.0, 0.0], -(0.6f64.asin().to_degrees()))?;
    cut(torus, above)
}
