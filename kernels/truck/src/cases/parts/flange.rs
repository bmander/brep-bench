//! "A round mounting flange on the z axis, its base on the plane z = 0: a plate, a disc of radius
//! 32 from z = 0 to z = 6; a hub, a cylinder of radius 15 standing on the plate from z = 6 to
//! z = 16; a bore of radius 8 on the z axis, right through the plate and the hub from z = 0 to
//! z = 16. Six bolt holes of radius 3 go straight through the plate along z. Their axes stand on a
//! circle of radius 24 about the z axis, one every 60° [starting at (24, 0)]. If you cut the holes
//! with cylinders, make them run past both faces of the plate."
use std::f64::consts::PI;
use crate::{shapes, Built, Refused};
use truck_modeling::*;

const BORE_RADIUS: f64 = 8.0;
const HUB_RADIUS: f64 = 15.0;
const PLATE_RADIUS: f64 = 32.0;
const PLATE_TOP: f64 = 6.0; // z; the plate stands on z = 0
const HUB_TOP: f64 = 16.0;
const BOLT_RADIUS: f64 = 3.0;
const BOLT_CIRCLE: f64 = 24.0;
const BOLTS: usize = 6; // one every 60°
/// How far each bolt hole's drill runs past the plate's faces.
const OVERRUN: f64 = 1.0;

pub fn build() -> Built {
    // the flange's section in the xz plane, bore, hub and plate in one outline: up the bore, out
    // across the hub's top, down its side, out across the plate's top, down its rim and back in
    // along its base
    let v = builder::vertices([
        Point3::new(BORE_RADIUS, 0.0, 0.0),
        Point3::new(BORE_RADIUS, 0.0, HUB_TOP),
        Point3::new(HUB_RADIUS, 0.0, HUB_TOP),
        Point3::new(HUB_RADIUS, 0.0, PLATE_TOP),
        Point3::new(PLATE_RADIUS, 0.0, PLATE_TOP),
        Point3::new(PLATE_RADIUS, 0.0, 0.0),
    ]);
    let outline: Wire = (0..v.len()).map(|i| builder::line(&v[i], &v[(i + 1) % v.len()])).collect();
    // run this way round, the section faces +y, the way a right-handed turn about z carries it
    let section: Face = builder::try_attach_plane(&[outline])?;
    // a whole turn, in four quarter turns
    let mut flange: Solid =
        builder::rsweep(&section, Point3::origin(), Vector3::unit_z(), Rad(2.0 * PI), 4);

    for k in 0..BOLTS {
        let angle = 2.0 * PI * k as f64 / BOLTS as f64;
        let (x, y) = (BOLT_CIRCLE * angle.cos(), BOLT_CIRCLE * angle.sin());
        let foot = Point3::new(x, y, -OVERRUN);
        let height = PLATE_TOP + 2.0 * OVERRUN;
        let mut drill = shapes::cylinder(foot, Vector3::unit_z(), BOLT_RADIUS, height);
        // truck-shapeops cuts by intersecting with the tool turned inside out
        drill.not();
        let tolerance = shapes::boolean_tolerance(&flange, &drill);
        flange = truck_shapeops::and(&flange, &drill, tolerance).ok_or_else(|| {
            let degrees = 360 * k / BOLTS;
            Refused::none_from(&format!("truck-shapeops `and` (the bolt hole at {degrees}°)"))
        })?;
    }
    Ok(flange.into())
}
