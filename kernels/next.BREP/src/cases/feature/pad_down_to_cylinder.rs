//! A cylinder of radius 10 along the x axis from x = −20 to x = 20, and a pad joined to it: the
//! square |x| ≤ 3, |y| ≤ 3 in the plane z = 15 extruded down until it meets the cylinder's side.
//!
//! The pad is the kernel's extrude up to a face (`extrude_profile_up_to_face`): the square slid
//! down until it meets the cylinder's side, picked by a point on it, so the pad's foot is that
//! side's own surface; the two are joined.
use crate::solids::boolean::union;
use crate::solids::shell::face_at;
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{extrude_profile_up_to_face, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cylinder = make_cylinder_brep(p(-20.0, 0.0, 0.0), p(1.0, 0.0, 0.0), 10.0, 40.0)?;
    let square = polygon(&[
        p(-3.0, -3.0, 15.0), p(3.0, -3.0, 15.0), p(3.0, 3.0, 15.0), p(-3.0, 3.0, 15.0),
    ])?;
    let side = face_at(&cylinder, p(0.0, 6.0, 8.0))?;
    let pad = extrude_profile_up_to_face(&square, p(0.0, 0.0, -1.0), &cylinder, side)?;
    Ok(union(&cylinder, &pad)?)
}
