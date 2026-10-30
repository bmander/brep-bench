//! The solid common to two cylinders of radius 5 and length 30 crossing at right angles, one
//! along the x axis and one parallel to y on the line x = 0, z = 9.99: they overlap 0.01 deep.
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const RADIUS: f64 = 5.0;
const LENGTH: f64 = 30.0;
const OFFSET: f64 = 9.99;

pub fn build() -> Result<Solid, crate::Refusal> {
    // vcad's cylinder stands on z from 0: turned 90° about y it runs along +x, and turned −90°
    // about x along +y
    let along_x = Solid::cylinder(RADIUS, LENGTH, SEGMENTS)
        .rotate(0.0, 90.0, 0.0)
        .translate(-LENGTH / 2.0, 0.0, 0.0);
    let along_y = Solid::cylinder(RADIUS, LENGTH, SEGMENTS)
        .rotate(-90.0, 0.0, 0.0)
        .translate(0.0, -LENGTH / 2.0, OFFSET);
    Ok(along_x.try_intersection(&along_y)?)
}
