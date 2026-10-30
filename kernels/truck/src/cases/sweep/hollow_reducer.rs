//! "A hollow square tube that tapers along the y axis, from y = 0 to y = 40. Its sections are
//! squares centred on the y axis, with sides parallel to the x and z axes, each with a square hole
//! at its centre leaving a wall 2 thick. At y = 0 the outside is 24 × 24 and the hole 20 × 20; at
//! y = 40 the outside is 12 × 12 and the hole 8 × 8. In between, the sizes run linearly with y, so
//! each corner, outside and in, runs in a straight line from one end to the other and the eight
//! long faces are flat trapezoids."
use truck_modeling::*;

const LENGTH: f64 = 40.0; // along y
const START_HALF: f64 = 12.0; // half the outside's side at y = 0
const END_HALF: f64 = 6.0; // at y = 40
const WALL: f64 = 2.0;

/// The square of half side `half` about the y axis in the plane `y`, run clockwise as seen from
/// +y, so that a face it bounds looks along -y.
fn square(y: f64, half: f64) -> Wire {
    let at = |x: f64, z: f64| Point3::new(x, y, z);
    let v = builder::vertices([at(-half, -half), at(half, -half), at(half, half), at(-half, half)]);
    (0..4).map(|i| builder::line(&v[i], &v[(i + 1) % 4])).collect()
}

pub fn build() -> crate::Built {
    let outside = (square(0.0, START_HALF), square(LENGTH, END_HALF));
    let hole = (square(0.0, START_HALF - WALL), square(LENGTH, END_HALF - WALL));
    // the long faces: `try_wire_homotopy` joins each side of one square to the same side of the
    // other by the straight lines between their points, so a corner runs straight from end to
    // end. Joined from the far end to the near, the outside faces look outward; the hole's faces,
    // joined from the near end to the far, look into the hole
    let outer: Shell = builder::try_wire_homotopy(&outside.1, &outside.0)?;
    let inner: Shell = builder::try_wire_homotopy(&hole.0, &hole.1)?;
    // the ends, on the very edges the long faces run to: the near end looks along -y, the far
    // end along +y
    let near: Face = builder::try_attach_plane(&[outside.0.clone(), hole.0.inverse()])?;
    let far: Face = builder::try_attach_plane(&[outside.1.inverse(), hole.1.clone()])?;
    let mut shell = outer;
    shell.extend(inner);
    shell.push(near);
    shell.push(far);
    let reducer = Solid::try_new(vec![shell])?;
    Ok(reducer.into())
}
