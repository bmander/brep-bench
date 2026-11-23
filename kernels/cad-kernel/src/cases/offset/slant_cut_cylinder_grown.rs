//! The cylinder of radius 3 on the z axis from its flat bottom at z = −10 up to where the plane
//! z = x·tan 45° cuts it, grown by 1 with rounded edges: every point within 1 of it.
use cad_kernel::build::{disc, kept, prism};
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

const RADIUS: f64 = 3.;
const BOTTOM: f64 = -10.;
const TILT_DEGREES: f64 = 45.;
const GROWTH: f64 = 1.;

pub fn build() -> Result<Brep, String> {
    // a cylinder reaching up past where the plane leaves it, at most 3 above the origin
    let cylinder = prism(&disc([0., 0., BOTTOM], [0., 0., 1.], RADIUS), 0., 2. * -BOTTOM)?;
    // kept below the plane through the origin that leans at the tilt about y, on the side of its
    // downward normal
    let tilt = TILT_DEGREES.to_radians();
    let slanted = kept(&cylinder, [0., 0., 0.], [tilt.sin(), 0., -tilt.cos()], 0.)?;
    grown(&slanted, GROWTH, Join::Round)
}
