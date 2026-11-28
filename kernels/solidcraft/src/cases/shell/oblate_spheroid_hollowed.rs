//! SolidCraft's Shell (`shell`) with no face to open: the oblate spheroid hollowed to a wall 1
//! thick round a closed cavity.
use super::prolate_spheroid_hollowed::spheroid;
use crate::solids::*;

pub fn build() -> R<Body> {
    solidcraft_kernel::shell(&spheroid(6.0, 3.0)?, &[], 1.0)
}
