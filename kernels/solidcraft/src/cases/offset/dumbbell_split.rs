//! A dumbbell plate, two 10 × 10 squares joined by a neck 4 long and 2 wide, extruded 6 along +z,
//! shrunk by 1.5: every point at least 1.5 from its outside. The neck's moved walls cross and are
//! trimmed away by SolidCraft's offset of the whole solid inward (`offset_solid`, rounded; its
//! self-intersections merged), leaving two bodies.
use crate::solids::*;

pub fn build() -> R<Body> {
    let outline = polygon(&[
        (0.0, 0.0),
        (10.0, 0.0),
        (10.0, 4.0),
        (14.0, 4.0),
        (14.0, 0.0),
        (24.0, 0.0),
        (24.0, 10.0),
        (14.0, 10.0),
        (14.0, 6.0),
        (10.0, 6.0),
        (10.0, 10.0),
        (0.0, 10.0),
    ]);
    let plate = extrude(&Plane::XY, outline, 0.0, 6.0)?;
    solidcraft_kernel::offset_solid(&plate, -1.5, true)
}
