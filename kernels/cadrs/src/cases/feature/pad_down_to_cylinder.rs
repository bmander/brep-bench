//! A cylinder of radius 10 along the x axis from x = −20 to x = 20, with a square pad on top: the
//! square |x| ≤ 3, |y| ≤ 3 in the plane z = 15 extruded down until it meets the cylinder's side,
//! and joined to it.
//!
//! The cylinder is a circle extruded along x. The pad is the square sketched on the plane
//! z = 15 and extruded down "up to face", the cylinder's side, so its bottom takes the side's
//! shape; the two are united.
use crate::bench::{cylinder, p3, union, v3};
use crate::shared::feature::extrude_to;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, ExtrudeEnd, Loop, Pick, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let cyl = cylinder(k, p3(-20.0, 0.0, 0.0), v3(1.0, 0.0, 0.0), 10.0, 40.0)?;
    let side = k.face_at(cyl, p3(0.0, 0.0, 10.0))?;
    let square = Profile::of(Plane::xy(15.0), vec![Loop::rectangle(-3.0, -3.0, 3.0, 3.0)]);
    let end = ExtrudeEnd::UpToFace { body: cyl, face: side, offset: 0.0 };
    let pad = extrude_to(k, &square, v3(0.0, 0.0, -1.0), end, &[])?;
    union(k, cyl, pad)
}
