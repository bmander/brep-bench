//! A square of side 2 swept along 350° of the arc of radius 10 about z, starting as the square from
//! (9, 0, −1) to (11, 0, 1) and staying square to the path: the flat ring between radii 9 and 11
//! from z = −1 to 1, from 0° to 350° about z.
use crate::sketch::polygon;
use cad_kernel::build::revolve;
use cad_kernel::topo::Brep;

const SPAN: f64 = 350.;

pub fn build() -> Result<Brep, String> {
    // a sweep along an arc is a revolution: the square in the xz plane, walked so that it faces +y,
    // the way the arc leaves it, turned 350° about z
    let section = polygon(&[[9., 0., -1.], [9., 0., 1.], [11., 0., 1.], [11., 0., -1.]]);
    revolve(&section, [0., 0., 0.], [0., 0., 1.], SPAN.to_radians())
}
