//! An L-shaped block shelled inward open at its step, the wall rising past the opening kept. The
//! outline through (0, 0), (20, 0), (20, 20), (15, 20), (15, 5) and (0, 5), extruded 20 along +z,
//! shelled inward to a wall 1 thick, open at the face y = 5, the corners sharp.
use crate::solids::*;

pub fn build() -> R<Body> {
    let outline = [(0.0, 0.0), (20.0, 0.0), (20.0, 20.0), (15.0, 20.0), (15.0, 5.0), (0.0, 5.0)];
    let block = extrude(&Plane::XY, polygon(&outline), 0.0, 20.0)?;
    solidcraft_kernel::shell(&block, &[v3(7.5, 5.0, 10.0)], 1.0)
}
