//! SolidCraft's Shell (`shell`) hollows a body inward to a wall of the thickness, opening the faces
//! picked by points on them.
use crate::solids::*;

const HEIGHT: f64 = 0.0;

pub fn build() -> R<Body> {
    let ball = solidcraft_kernel::sphere(Vec3::ZERO, 5.0)?;
    // everything above the plane z = HEIGHT cut away
    let below = cut(&ball, &block((-10.0, -10.0, HEIGHT), (10.0, 10.0, 10.0))?)?;
    solidcraft_kernel::shell(&below, &[v3(0.0, 0.0, HEIGHT)], 1.0)
}
