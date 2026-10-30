//! "In the xz plane, take the rectangle with x from 6 to 10 and z from 0 to 4. Revolve it 90°
//! about the z axis, right-handed: from +x towards +y."
use truck_modeling::*;

const INNER: f64 = 6.0;
const OUTER: f64 = 10.0;
const HEIGHT: f64 = 4.0;
const ANGLE: f64 = 90.0; // degrees

pub fn build() -> crate::Built {
    let v = builder::vertices([
        Point3::new(INNER, 0.0, 0.0),
        Point3::new(INNER, 0.0, HEIGHT),
        Point3::new(OUTER, 0.0, HEIGHT),
        Point3::new(OUTER, 0.0, 0.0),
    ]);
    let rectangle: Wire = (0..4).map(|i| builder::line(&v[i], &v[(i + 1) % 4])).collect();
    // run this way round, the rectangle faces +y: the way a right-handed turn about z carries it,
    // or the solid comes out inside out
    let section: Face = builder::try_attach_plane(&[rectangle])?;
    let turn = Rad(ANGLE.to_radians());
    let sector: Solid = builder::rsweep(&section, Point3::origin(), Vector3::unit_z(), turn, 1);
    Ok(sector.into())
}
