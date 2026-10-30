//! "A hollow square duct bent through a quarter turn. Its centreline is an arc of radius 30 in the
//! xy plane about the z axis, from (30, 0, 0) to (0, 30, 0). [The section] is a square 18 × 18
//! centred on the arc, two of its sides horizontal (at z = −9 and z = 9) and two vertical (at 21
//! and 39 from the z axis), with a square hole 14 × 14 at its centre: the duct is that hollow
//! square, standing in the xz plane at x = 30, turned a quarter turn about the z axis, from +x
//! towards +y."
use truck_modeling::*;

const CENTRELINE_RADIUS: f64 = 30.0;
const OUTER_HALF: f64 = 9.0; // half the side of the outside square
const WALL: f64 = 2.0;
const TURN: f64 = 90.0; // degrees

/// The square of half side `half` about the centreline's start (30, 0, 0) in the xz plane, run
/// counterclockwise as seen from +y.
fn square(half: f64) -> Wire {
    let at = |x: f64, z: f64| Point3::new(CENTRELINE_RADIUS + x, 0.0, z);
    let v = builder::vertices([at(-half, -half), at(-half, half), at(half, half), at(half, -half)]);
    (0..4).map(|i| builder::line(&v[i], &v[(i + 1) % 4])).collect()
}

pub fn build() -> crate::Built {
    // the outside square counterclockwise seen from +y and the hole's square the other way round,
    // so the hollow section faces +y, the way a right-handed turn about z carries it
    let outside = square(OUTER_HALF);
    let hole = square(OUTER_HALF - WALL).inverse();
    let section: Face = builder::try_attach_plane(&[outside, hole])?;
    // truck sweeps by rotation, and this centreline is an arc about the z axis: the duct is the
    // section turned a quarter turn about z, its ends the section and its image
    let turn = Rad(TURN.to_radians());
    let elbow: Solid = builder::rsweep(&section, Point3::origin(), Vector3::unit_z(), turn, 1);
    Ok(elbow.into())
}
