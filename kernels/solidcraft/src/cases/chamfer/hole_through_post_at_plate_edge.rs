//! The plate from (−50, −50, 0) to (50, 50, 20) joined to the post, the cylinder of radius 15
//! about the vertical line through (45, 0) from z = 0 to z = 40, which overhangs the plate's side
//! x = 50. A hole of radius 6 is drilled right through along the post's axis, and both its rims,
//! on the bottom face and on the post's top, are chamfered 1.5 on both faces.
use crate::solids::*;

pub fn build() -> R<Body> {
    let plate = block((-50.0, -50.0, 0.0), (50.0, 50.0, 20.0))?;
    let post = union(&plate, &zcyl(45.0, 0.0, 15.0, 0.0, 40.0)?)?;
    // the drill runs 1 past both ends
    let drilled = cut(&post, &zcyl(45.0, 0.0, 6.0, -1.0, 41.0)?)?;
    // each rim picked at its point nearest −x, inside the plate
    solidcraft_kernel::chamfer(&drilled, &[v3(39.0, 0.0, 0.0), v3(39.0, 0.0, 40.0)], 1.5)
}
