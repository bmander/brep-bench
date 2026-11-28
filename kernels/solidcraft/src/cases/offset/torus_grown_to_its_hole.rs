//! The torus whose tube of radius 2 runs about a middle circle of radius 4, grown by 2: every
//! point within 2 of it, the tube of radius 4 about the same circle, the horn torus, which closes
//! the hole and touches itself at the origin. SolidCraft's offset of the whole solid
//! (`offset_solid`), rounded: the torus has no edge, so that is its one face moved 2 outward.
use crate::solids::*;

pub fn build() -> R<Body> {
    let torus = solidcraft_kernel::torus(Vec3::ZERO, 4.0, 2.0)?;
    solidcraft_kernel::offset_solid(&torus, 2.0, true)
}
