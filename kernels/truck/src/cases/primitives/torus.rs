//! "A solid torus centred on the origin, about the z axis. The circle through the middle of its
//! tube has radius 10, and the tube has radius 2."
use std::f64::consts::PI;
use truck_modeling::*;

const RING_RADIUS: f64 = 10.0;
const TUBE_RADIUS: f64 = 2.0;

// workaround: no torus primitive
pub fn build() -> crate::Built {
    // as truck's `torus` example: the tube's section, a circle in the xz plane about the point
    // on the ring at +x, from its top round in four arcs
    let middle = Point3::new(RING_RADIUS, 0.0, 0.0);
    let top = builder::vertex(Point3::new(RING_RADIUS, 0.0, TUBE_RADIUS));
    let section: Wire = builder::rsweep(&top, middle, Vector3::unit_y(), Rad(2.0 * PI), 4);
    // turned once about the z axis, the circle sweeps out the closed shell of the torus
    let shell = builder::rsweep(&section, Point3::origin(), Vector3::unit_z(), Rad(2.0 * PI), 4);
    let torus = Solid::new(vec![shell]);
    Ok(torus.into())
}
