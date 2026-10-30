//! "A solid cylinder of radius 5 and height 10 whose axis is the z axis, standing on the xy
//! plane: it runs from z = 0 to z = 10."
use std::f64::consts::PI;
use truck_modeling::*;

const RADIUS: f64 = 5.0;
const HEIGHT: f64 = 10.0;

pub fn build() -> crate::Built {
    // the circle about the z axis, from +x round in four arcs (truck has no closed edge)
    let rim = builder::vertex(Point3::new(RADIUS, 0.0, 0.0));
    let circle: Wire =
        builder::rsweep(&rim, Point3::origin(), Vector3::unit_z(), Rad(2.0 * PI), 4);
    // it runs counterclockwise seen from +z, so the disc faces up, the way it is swept
    let disc: Face = builder::try_attach_plane(&[circle])?;
    let cylinder: Solid = builder::tsweep(&disc, Vector3::new(0.0, 0.0, HEIGHT));
    Ok(cylinder.into())
}
