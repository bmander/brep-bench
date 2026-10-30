//! Two cylinders crossing at right angles, united: the main of radius 5 along the x axis from
//! x = −15 to 15, the branch of radius 3 parallel to y from y = −15 to 15 on the line x = 0,
//! z = 2. The branch touches the main's crown at (0, 0, 5).
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const MAIN_RADIUS: f64 = 5.0;
const BRANCH_RADIUS: f64 = 3.0;
const LENGTH: f64 = 30.0;
const BRANCH_Z: f64 = 2.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    // vcad's cylinder stands on z from 0: turned 90° about y it runs along +x, and turned −90°
    // about x along +y
    let main = Solid::cylinder(MAIN_RADIUS, LENGTH, SEGMENTS)
        .rotate(0.0, 90.0, 0.0)
        .translate(-LENGTH / 2.0, 0.0, 0.0);
    let branch = Solid::cylinder(BRANCH_RADIUS, LENGTH, SEGMENTS)
        .rotate(-90.0, 0.0, 0.0)
        .translate(0.0, -LENGTH / 2.0, BRANCH_Z);
    Ok(main.try_union(&branch)?)
}
