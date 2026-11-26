//! The cube from (−10, −10, −10) to (10, 10, 10) less the oblate spheroid of
//! `geometry/oblate-spheroid`, a closed cavity at its middle, grown by 1 to its parallel body.
use crate::cases::geometry::r#oblate_spheroid;
use crate::solids::boolean::cut;
use crate::solids::offset::parallel;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cube = make_box_brep(p(-10.0, -10.0, -10.0), 20.0, 20.0, 20.0)?;
    Ok(parallel(&cut(&cube, &oblate_spheroid::build()?)?, 1.0)?)
}
