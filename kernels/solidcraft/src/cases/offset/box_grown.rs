//! The box from the origin to (10, 8, 6) grown by 1: every point within 1 of it, its faces moved
//! out, its edges quarter cylinders and its corners eighths of a ball, SolidCraft's offset of the
//! whole solid (`offset_solid`), its convex edges rounded.
use crate::solids::*;

pub fn build() -> R<Body> {
    let block = block((0.0, 0.0, 0.0), (10.0, 8.0, 6.0))?;
    solidcraft_kernel::offset_solid(&block, 1.0, true)
}
