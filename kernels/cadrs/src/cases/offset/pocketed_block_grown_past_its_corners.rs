//! The block from (−20, −15, 0) to (20, 15, 10) with a pocket cut down into its top to a flat
//! floor at z = 4, the rectangle from (−10, −7) to (10, 7) with its four corners rounded to
//! radius 2, grown by 3, more than the corners' radius: the solid of every point within 3 of it.
//!
//! The block a rectangle extruded, the pocket its rounded rectangle sketched at the floor and
//! extruded up past the top, and cut; then cadrs's offset with rounded edges.
use crate::bench::{block, cut, extrude, p3};
use crate::shared::fillet::rounded_polygon;
use crate::shared::offset::grown;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = block(k, p3(-20.0, -15.0, 0.0), p3(20.0, 15.0, 10.0))?;
    let outline = rounded_polygon(&[(-10.0, -7.0), (10.0, -7.0), (10.0, 7.0), (-10.0, 7.0)], &[2.0; 4]);
    let pocket = extrude(k, &Profile::of(Plane::xy(4.0), vec![Loop::new(outline)]), 7.0)?;
    let pocketed = cut(k, b, pocket)?;
    grown(k, pocketed, 3.0)
}
