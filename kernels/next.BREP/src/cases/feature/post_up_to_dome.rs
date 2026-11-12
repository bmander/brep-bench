//! A closed hollow dome, the disk of radius 20 from z = −2 to z = 0 and on it the hemispherical
//! shell between 18 and 20 from the origin, its cavity the half ball of radius 18 above the plate;
//! and a post, the disk of radius 3 on the cavity's floor extruded up until it meets the inside of
//! the dome, joined to it.
//!
//! The half balls are the kernel's balls (poles on the z axis) less the block under z = 0. The
//! dome is the plate joined to the half ball of radius 20, less the half ball of radius 18.
//! The post is the kernel's extrude up to a face (`extrude_profile_up_to_face`): the circle of
//! radius 3 on the floor slid up until it meets the inside of the dome, picked by a point on it, so
//! the post's head is that sphere's own surface; the two are joined.
use crate::solids::boolean::{cut, union};
use crate::solids::p;
use crate::Refusal;
use crate::solids::shell::face_at;
use brep_kernel::{
    extrude_profile_up_to_face, make_box_brep, make_circle, make_cylinder_brep, make_sphere_brep,
    BrepSolid,
};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (o, z) = (p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let below = make_box_brep(p(-30.0, -30.0, -30.0), 60.0, 60.0, 30.0)?;
    let half_ball = |r: f64| -> Result<BrepSolid, Refusal> {
        Ok(cut(&make_sphere_brep(o, r, z)?, &below)?)
    };
    let plate = make_cylinder_brep(p(0.0, 0.0, -2.0), z, 20.0, 2.0)?;
    let dome = cut(&union(&plate, &half_ball(20.0)?)?, &half_ball(18.0)?)?;
    let inside = face_at(&dome, p(8.0, 6.0, 224f64.sqrt()))?;
    let post = extrude_profile_up_to_face(&[make_circle(o, z, 3.0)?], z, &dome, inside)?;
    Ok(union(&dome, &post)?)
}
