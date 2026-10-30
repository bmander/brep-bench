//! "A circle of radius 2 swept along a quarter turn of an arc. The arc lies in the xy plane about
//! the z axis, with radius 10, and runs from (10, 0, 0) to (0, 10, 0). The section stays square to
//! the path, and the ends are flat."
use std::f64::consts::PI;
use truck_modeling::*;

const SECTION_RADIUS: f64 = 2.0;
const PATH_RADIUS: f64 = 10.0;
const PATH_TURN: f64 = 90.0; // degrees

pub fn build() -> crate::Built {
    // the section: the circle about the path's start (10, 0, 0) in the xz plane, square to the
    // path there, from +x round in four arcs
    let start = Point3::new(PATH_RADIUS, 0.0, 0.0);
    let rim = builder::vertex(start + Vector3::unit_x() * SECTION_RADIUS);
    let circle: Wire = builder::rsweep(&rim, start, Vector3::unit_y(), Rad(2.0 * PI), 4);
    // counterclockwise about +y, the way the path leaves (10, 0, 0), so the disc faces along it
    let section: Face = builder::try_attach_plane(&[circle])?;
    // workaround: no sweep along a path
    // truck sweeps by translation and by rotation, and this path is an arc about the z axis: the
    // pipe is the section turned a quarter turn about z, its ends the section and its image
    let turn = Rad(PATH_TURN.to_radians());
    let pipe: Solid = builder::rsweep(&section, Point3::origin(), Vector3::unit_z(), turn, 1);
    Ok(pipe.into())
}
