//! The plus-shaped plate, the outline through (3, 0), (6, 0), (6, 3), (9, 3), (9, 6), (6, 6),
//! (6, 9), (3, 9), (3, 6), (0, 6), (0, 3) and (3, 3) extruded 4 along +z, shrunk by 1 to its
//! inner parallel body: its four concave vertical edges rounded, the rest sharp.
use crate::solids::offset::parallel;
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let outline = polygon(&[
        p(3.0, 0.0, 0.0), p(6.0, 0.0, 0.0), p(6.0, 3.0, 0.0), p(9.0, 3.0, 0.0),
        p(9.0, 6.0, 0.0), p(6.0, 6.0, 0.0), p(6.0, 9.0, 0.0), p(3.0, 9.0, 0.0),
        p(3.0, 6.0, 0.0), p(0.0, 6.0, 0.0), p(0.0, 3.0, 0.0), p(3.0, 3.0, 0.0),
    ])?;
    let plate = extrude_profile_brep(&outline, p(0.0, 0.0, 1.0), 4.0)?;
    Ok(parallel(&plate, -1.0)?)
}
