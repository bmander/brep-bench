//! A block (80 by 60 by 16) with a pocket cut 10 into its top (the rectangle from (−25, −15) to
//! (25, 15), its corners rounded to radius 4), the rim of the pocket's floor rounded to radius 4:
//! the corners' own radius, so the round at each corner is an eighth of a ball.
//!
//! The block is a rectangle extruded; the pocket its rounded outline on the plane z = 6
//! extruded 10, cut away; then cadrs's Fillet on the eight edges of the floor's rim.
use crate::bench::{block, cut, extrude, p3};
use crate::shared::fillet::{edges_at_height, fillet, rounded_polygon};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = block(k, p3(-40.0, -30.0, 0.0), p3(40.0, 30.0, 16.0))?;
    let outline = rounded_polygon(&[(-25.0, -15.0), (25.0, -15.0), (25.0, 15.0), (-25.0, 15.0)], &[4.0; 4]);
    let pocket = extrude(k, &Profile::of(Plane::xy(6.0), vec![Loop::new(outline)]), 10.0)?;
    let cut = cut(k, b, pocket)?;
    let rim = edges_at_height(k, cut, 6.0)?;
    fillet(k, cut, &rim, 4.0)
}
