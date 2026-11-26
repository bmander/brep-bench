//! A circle of radius 2 swept along 350° of the arc of radius 10 about z, square to the path, its
//! ends flat: the solid torus's part from 0° to 350° about z.
use cad_kernel::build::{disc, revolve};
use cad_kernel::topo::Brep;

const ARC_RADIUS: f64 = 10.;
const RADIUS: f64 = 2.;
const SPAN: f64 = 350.;

pub fn build() -> Result<Brep, String> {
    // a sweep along an arc is a revolution: the section square to the arc where it starts (the
    // disc in the xz plane about (10, 0, 0)) turned 350° about z
    let section = disc([ARC_RADIUS, 0., 0.], [0., 1., 0.], RADIUS);
    revolve(&section, [0., 0., 0.], [0., 0., 1.], SPAN.to_radians())
}
