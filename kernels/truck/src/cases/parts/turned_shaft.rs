//! "A stepped shaft on the z axis, the solid swept by turning a section in the xz plane a full turn
//! about the z axis. [The section runs] from (0, 0) out to (20, 0), up to (20, 60), in to
//! (16, 60), along a quarter circle of radius 4 centred at (16, 64) to (12, 64), bulging toward the
//! corner (12, 60), up to (12, 160), in to (0, 160) and down the axis back to (0, 0)."
use std::f64::consts::{FRAC_1_SQRT_2, PI};
use truck_modeling::*;

const THICK_RADIUS: f64 = 20.0;
const THIN_RADIUS: f64 = 12.0;
const SHOULDER: f64 = 60.0; // z
const LENGTH: f64 = 160.0;
const ROUND_RADIUS: f64 = 4.0;

pub fn build() -> crate::Built {
    // the round's centre, (16, 64), and the middle of its arc, out from the centre toward the
    // corner (12, 60)
    let centre = Point3::new(THIN_RADIUS + ROUND_RADIUS, 0.0, SHOULDER + ROUND_RADIUS);
    let middle = centre + Vector3::new(-1.0, 0.0, -1.0) * (ROUND_RADIUS * FRAC_1_SQRT_2);
    // the meridian, from the top of the axis down the outline to its foot; `builder::cone` turns
    // it once about the z axis and closes the ends that lie on the axis. It runs down, against the
    // axis, or the solid comes out inside out
    let v = builder::vertices([
        Point3::new(0.0, 0.0, LENGTH),
        Point3::new(THIN_RADIUS, 0.0, LENGTH),
        Point3::new(THIN_RADIUS, 0.0, SHOULDER + ROUND_RADIUS),
        Point3::new(THIN_RADIUS + ROUND_RADIUS, 0.0, SHOULDER),
        Point3::new(THICK_RADIUS, 0.0, SHOULDER),
        Point3::new(THICK_RADIUS, 0.0, 0.0),
        Point3::origin(),
    ]);
    let meridian: Wire = vec![
        builder::line(&v[0], &v[1]),
        builder::line(&v[1], &v[2]),
        builder::circle_arc(&v[2], &v[3], middle),
        builder::line(&v[3], &v[4]),
        builder::line(&v[4], &v[5]),
        builder::line(&v[5], &v[6]),
    ]
    .into();
    let shell = builder::cone(&meridian, Vector3::unit_z(), Rad(2.0 * PI), 4);
    let shaft = Solid::new(vec![shell]);
    Ok(shaft.into())
}
