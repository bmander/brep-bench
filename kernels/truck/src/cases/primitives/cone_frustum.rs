//! "A truncated cone on the z axis, from z = 0 to z = 6, of radius 5 at its base (z = 0) and
//! radius 2 at its top (z = 6)."
use std::f64::consts::PI;
use truck_modeling::*;

const BASE_RADIUS: f64 = 5.0;
const TOP_RADIUS: f64 = 2.0;
const HEIGHT: f64 = 6.0;

// workaround: no cone primitive
pub fn build() -> crate::Built {
    // the meridian in the xz plane: from the top of the axis out to the top rim, down the slant
    // to the base rim and back in to the foot of the axis
    let v = builder::vertices([
        Point3::new(0.0, 0.0, HEIGHT),
        Point3::new(TOP_RADIUS, 0.0, HEIGHT),
        Point3::new(BASE_RADIUS, 0.0, 0.0),
        Point3::origin(),
    ]);
    let meridian: Wire = vec![
        builder::line(&v[0], &v[1]),
        builder::line(&v[1], &v[2]),
        builder::line(&v[2], &v[3]),
    ]
    .into();
    // `builder::cone` turns it once about the z axis and closes the ends that lie on the axis;
    // the meridian runs down, against the axis, or the solid comes out inside out
    let shell = builder::cone(&meridian, Vector3::unit_z(), Rad(2.0 * PI), 4);
    let frustum = Solid::new(vec![shell]);
    Ok(frustum.into())
}
