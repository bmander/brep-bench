//! "An L-shaped plate bracket, 10 thick: its outline lies in the xz plane and is extruded from
//! y = −5 to y = 5. [The outline runs] from (0, 0) to (20, 0), up to (20, 92), along a quarter
//! circle of radius 8 centred at (28, 92) to (28, 100), bulging toward the corner (20, 100), to
//! (100, 100), up to (100, 120), back to (0, 120) and down to (0, 0)."
use std::f64::consts::FRAC_1_SQRT_2;
use truck_modeling::*;

const POST_WIDTH: f64 = 20.0; // in x
const ARM_REACH: f64 = 100.0; // x
const ARM_UNDERSIDE: f64 = 100.0; // z
const TOP: f64 = 120.0;
const ROUND_RADIUS: f64 = 8.0;
const HALF_THICKNESS: f64 = 5.0; // in y, either side of y = 0

pub fn build() -> crate::Built {
    // the outline drawn in the plane y = 5, (x, z) as Point3::new(x, 5, z)
    let at = |x: f64, z: f64| Point3::new(x, HALF_THICKNESS, z);
    let v = builder::vertices([
        at(0.0, 0.0),
        at(POST_WIDTH, 0.0),
        at(POST_WIDTH, ARM_UNDERSIDE - ROUND_RADIUS),
        at(POST_WIDTH + ROUND_RADIUS, ARM_UNDERSIDE),
        at(ARM_REACH, ARM_UNDERSIDE),
        at(ARM_REACH, TOP),
        at(0.0, TOP),
    ]);
    // the round's centre, (28, 92), and the middle of its arc, out from the centre toward the
    // corner (20, 100)
    let centre = at(POST_WIDTH + ROUND_RADIUS, ARM_UNDERSIDE - ROUND_RADIUS);
    let middle = centre + Vector3::new(-1.0, 0.0, 1.0) * (ROUND_RADIUS * FRAC_1_SQRT_2);
    let outline: Wire = vec![
        builder::line(&v[0], &v[1]),
        builder::line(&v[1], &v[2]),
        builder::circle_arc(&v[2], &v[3], middle),
        builder::line(&v[3], &v[4]),
        builder::line(&v[4], &v[5]),
        builder::line(&v[5], &v[6]),
        builder::line(&v[6], &v[0]),
    ]
    .into();
    // the outline runs clockwise seen from +y, so the face looks along -y: it is drawn at
    // y = 5 and swept the way it faces, to y = -5
    let profile: Face = builder::try_attach_plane(&[outline])?;
    let bracket: Solid = builder::tsweep(&profile, Vector3::new(0.0, -2.0 * HALF_THICKNESS, 0.0));
    Ok(bracket.into())
}
