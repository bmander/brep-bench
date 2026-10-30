//! "In the xz plane, take the rectangle with x from 6 to 10 and z from 0 to 4. Revolve it a full
//! turn about the z axis."
use std::f64::consts::PI;
use truck_modeling::*;

const INNER: f64 = 6.0;
const OUTER: f64 = 10.0;
const HEIGHT: f64 = 4.0;

pub fn build() -> crate::Built {
    let v = builder::vertices([
        Point3::new(INNER, 0.0, 0.0),
        Point3::new(INNER, 0.0, HEIGHT),
        Point3::new(OUTER, 0.0, HEIGHT),
        Point3::new(OUTER, 0.0, 0.0),
    ]);
    let rectangle: Wire = (0..4).map(|i| builder::line(&v[i], &v[(i + 1) % 4])).collect();
    // run this way round, the rectangle faces +y, the way a right-handed turn about z carries it
    let section: Face = builder::try_attach_plane(&[rectangle])?;
    // a whole turn, in four quarter turns
    let washer: Solid =
        builder::rsweep(&section, Point3::origin(), Vector3::unit_z(), Rad(2.0 * PI), 4);
    Ok(washer.into())
}
