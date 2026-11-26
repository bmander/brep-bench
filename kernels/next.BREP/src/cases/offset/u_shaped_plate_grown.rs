//! The U-shaped plate, the outline through (0, 0), (9, 0), (9, 9), (6, 9), (6, 3), (3, 3),
//! (3, 9) and (0, 9) extruded 2 along +z, grown by 1 to its parallel body: its convex edges
//! rounded, the slot's two concave vertical edges left sharp.
use crate::solids::offset::parallel;
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let outline = polygon(&[
        p(0.0, 0.0, 0.0), p(9.0, 0.0, 0.0), p(9.0, 9.0, 0.0), p(6.0, 9.0, 0.0),
        p(6.0, 3.0, 0.0), p(3.0, 3.0, 0.0), p(3.0, 9.0, 0.0), p(0.0, 9.0, 0.0),
    ])?;
    let plate = extrude_profile_brep(&outline, p(0.0, 0.0, 1.0), 2.0)?;
    Ok(parallel(&plate, 1.0)?)
}
