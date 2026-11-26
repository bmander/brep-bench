//! The plate from (−20, −15, 0) to (20, 15, 4) with a window cut through it along z, the
//! rectangle from (−10, −7) to (10, 7) with its four corners rounded to radius 2, grown by 3, more
//! than the corners' radius: the solid of every point within 3 of it.
//!
//! The plate a rectangle extruded, the window its rounded rectangle sketched below the plate and
//! extruded up through it, and cut; then cadrs's offset with rounded edges.
use crate::bench::{block, cut, extrude, p3};
use crate::shared::fillet::rounded_polygon;
use crate::shared::offset::grown;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let plate = block(k, p3(-20.0, -15.0, 0.0), p3(20.0, 15.0, 4.0))?;
    let outline = rounded_polygon(&[(-10.0, -7.0), (10.0, -7.0), (10.0, 7.0), (-10.0, 7.0)], &[2.0; 4]);
    let window = extrude(k, &Profile::of(Plane::xy(-1.0), vec![Loop::new(outline)]), 6.0)?;
    let windowed = cut(k, plate, window)?;
    grown(k, windowed, 3.0)
}
