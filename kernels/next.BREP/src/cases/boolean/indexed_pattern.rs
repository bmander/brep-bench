//! A disc of radius 20 on the z axis, from z = −5 to z = 0, with six holes of radius 2 through it
//! on the circle of radius 12: the first at (12, 0), the others it turned about the z axis by
//! 60°, 120°, … 300°, cut one after another by cylinders from z = −6 to z = 1.
use crate::solids::boolean::{cut, turned_about_z};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let mut disc = make_cylinder_brep(p(0.0, 0.0, -5.0), p(0.0, 0.0, 1.0), 20.0, 5.0)?;
    let first = make_cylinder_brep(p(12.0, 0.0, -6.0), p(0.0, 0.0, 1.0), 2.0, 7.0)?;
    for k in 0..6 {
        let drill = turned_about_z(&first, (60.0 * k as f64).to_radians())?;
        disc = cut(&disc, &drill)?;
    }
    Ok(disc)
}
