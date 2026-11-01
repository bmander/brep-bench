//! SolidCraft's Shell (`shell`) hollows a body inward to a wall of the thickness, opening the faces
//! picked by points on them. Its walls are moved and extended until they meet: at the concave edge
//! x = 5, y = 5 the moved walls x = 4 and y = 4 meet in the line x = 4, y = 4, a convex edge of
//! the wall's inside. Rounded to radius 1, that edge's round is tangent to x = 4 and y = 4 about the
//! line x = 5, y = 5: the quarter cylinder of radius 1 about the concave edge, which is what the
//! wall within 1 of the plate's faces is there.
use crate::solids::*;

pub fn build() -> R<Body> {
    let outline = [(0.0, 0.0), (20.0, 0.0), (20.0, 5.0), (5.0, 5.0), (5.0, 15.0), (0.0, 15.0)];
    let plate = extrude(&Plane::XY, polygon(&outline), 0.0, 8.0)?;
    let shelled = solidcraft_kernel::shell(&plate, &[v3(2.0, 2.0, 8.0)], 1.0)?;
    solidcraft_kernel::fillet(&shelled, &[v3(4.0, 4.0, 4.0)], 1.0)
}
