//! The union of two cylinders of radius 5 and length 30 crossing at right angles, one along the x
//! axis from x = −15 to 15 and one along the y axis from y = −15 to 15.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const RADIUS: f64 = 5.0;
const LENGTH: f64 = 30.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    // vcad's cylinder stands on z: centre it on the origin, then turn it onto x and onto y
    let centred = Solid::cylinder(RADIUS, LENGTH, SEGMENTS).translate(0.0, 0.0, -LENGTH / 2.0);
    let along_x = centred.rotate(0.0, 90.0, 0.0);
    let along_y = centred.rotate(90.0, 0.0, 0.0);
    Ok(along_x.try_union(&along_y)?)
}
