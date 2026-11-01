//! An L-shaped plate bracket: its outline in the xz plane, the post 20 wide and 120 tall and the
//! arm 20 deep reaching to x = 100 along its top, the inside corner between them the quarter
//! circle of radius 8 about (28, 92), extruded from y = −5 to y = 5.
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, make_arc, BrepSolid};
use std::f64::consts::FRAC_PI_2;

pub fn build() -> Result<BrepSolid, Refusal> {
    // The outline in the plane y = −5: (x, z) as (x, −5, z).
    let xz = |x: f64, z: f64| p(x, -5.0, z);
    let mut outline = polygon(&[xz(0.0, 0.0), xz(20.0, 0.0), xz(20.0, 92.0)])?;
    outline.pop(); // the polygon's closing side; the outline goes on round the arc
    // From (20, 92) to (28, 100) about (28, 92), bulging toward the corner (20, 100): angle 0 along
    // −x, a quarter turn along +z.
    let (minus_x, plus_z) = (p(-1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    outline.push(make_arc(xz(28.0, 92.0), minus_x, plus_z, 8.0, 0.0, FRAC_PI_2)?);
    let mut rest = polygon(&[
        xz(28.0, 100.0), xz(100.0, 100.0), xz(100.0, 120.0), xz(0.0, 120.0), xz(0.0, 0.0),
    ])?;
    rest.pop();
    outline.extend(rest);
    Ok(extrude_profile_brep(&outline, p(0.0, 1.0, 0.0), 10.0)?)
}
