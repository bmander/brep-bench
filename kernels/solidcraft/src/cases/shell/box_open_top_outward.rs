//! The box from the origin to (10, 8, 6) shelled outward to a wall 1 thick, open at its top face,
//! z = 6, the wall every point outside the box within 1 of its other faces: the box grown by 1 and
//! cut off at z = 6, less the box. Its outside is rounded at the box's convex edges and corners.
//!
//! The cup is made as the description composes it: SolidCraft's offset of the whole box
//! (`offset_solid`), its convex edges rounded, cut off at the plane of the top and hollowed by the
//! box.
use crate::solids::*;

pub fn build() -> R<Body> {
    let box_ = block((0.0, 0.0, 0.0), (10.0, 8.0, 6.0))?;
    let grown = solidcraft_kernel::offset_solid(&box_, 1.0, true)?;
    let above = block((-2.0, -2.0, 6.0), (12.0, 10.0, 8.0))?;
    cut(&cut(&grown, &above)?, &box_)
}
