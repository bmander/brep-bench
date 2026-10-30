//! "A pulley turned about the z axis, symmetric about the plane z = 0: a hub, a cylinder of radius
//! 12 from z = −14 to z = 14; a rim, a disc of radius 28 from z = −9 to z = 9; a bore of radius 6
//! on the z axis, right through. A V-shaped belt groove runs all the way round the rim: its root
//! is the circle of radius 22 at z = 0, and its two flanks are cones that open 5 in z for every 7
//! outward in radius, meeting the rim's face (radius 28) at z = ±30/7."
//!
//! The groove is drawn into the section that is turned, not cut: truck-shapeops' `and` of the
//! turned body and the turned triangle (22, 0), (29, −5), (29, 5) ran for ten minutes without an
//! answer.
use std::f64::consts::PI;
use truck_modeling::*;

const BORE_RADIUS: f64 = 6.0;
const HUB_RADIUS: f64 = 12.0;
const HUB_HALF_LENGTH: f64 = 14.0; // along z, either side of z = 0
const RIM_RADIUS: f64 = 28.0;
const RIM_HALF_LENGTH: f64 = 9.0;
const GROOVE_ROOT: f64 = 22.0; // the groove's root, at z = 0
const FLANK_SLOPE: f64 = 5.0 / 7.0; // the flanks' rise in z per unit of radius

pub fn build() -> crate::Built {
    // the section in the xz plane, (x, z): up the bore, out across the hub's end, down its side to
    // the rim, out across the rim's face, down its edge to the groove, in along one flank to the
    // root and out along the other, and back the same way below
    let edge = (RIM_RADIUS - GROOVE_ROOT) * FLANK_SLOPE; // where a flank meets the rim's face
    let (bore, hub, rim) = (BORE_RADIUS, HUB_RADIUS, RIM_RADIUS);
    let (hub_half, rim_half) = (HUB_HALF_LENGTH, RIM_HALF_LENGTH);
    let outline = [
        (bore, -hub_half),
        (bore, hub_half),
        (hub, hub_half),
        (hub, rim_half),
        (rim, rim_half),
        (rim, edge),
        (GROOVE_ROOT, 0.0),
        (rim, -edge),
        (rim, -rim_half),
        (hub, -rim_half),
        (hub, -hub_half),
    ];
    let v = builder::vertices(outline.map(|(x, z)| Point3::new(x, 0.0, z)));
    let wire: Wire = (0..v.len()).map(|i| builder::line(&v[i], &v[(i + 1) % v.len()])).collect();
    // run this way round, the section faces +y, the way a right-handed turn about z carries it
    let section: Face = builder::try_attach_plane(&[wire])?;
    // a whole turn, in four quarter turns
    let pulley: Solid =
        builder::rsweep(&section, Point3::origin(), Vector3::unit_z(), Rad(2.0 * PI), 4);
    Ok(pulley.into())
}
