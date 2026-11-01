//! SolidCraft's Shell (`shell`) hollows a body inward to a wall of the thickness, opening the faces
//! picked by points on them. Its walls are moved and extended until they meet, so the concave edge's two
//! walls, moved, meet in a sharp edge.
use crate::solids::*;

pub fn build() -> R<Body> {
    let outline = [(0.0, 0.0), (20.0, 0.0), (20.0, 5.0), (5.0, 5.0), (5.0, 15.0), (0.0, 15.0)];
    let plate = extrude(&Plane::XY, polygon(&outline), 0.0, 8.0)?;
    solidcraft_kernel::shell(&plate, &[v3(2.0, 2.0, 8.0)], 1.0)
}
