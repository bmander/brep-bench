//! The rectangle x 6..10, z 0..4 in the xz plane revolved 90° about z, from +x towards +y.
use crate::sketch::polygon;
use cad_kernel::build::revolve;
use cad_kernel::topo::Brep;

const INNER: f64 = 6.;
const OUTER: f64 = 10.;
const HEIGHT: f64 = 4.;
const TURN_DEGREES: f64 = 90.;

pub fn build() -> Result<Brep, String> {
    let section = polygon(&[[INNER, 0., 0.], [OUTER, 0., 0.], [OUTER, 0., HEIGHT], [INNER, 0., HEIGHT]]);
    // a positive angle turns right-handed about the axis: from +x towards +y about +z
    revolve(&section, [0., 0., 0.], [0., 0., 1.], TURN_DEGREES.to_radians())
}
