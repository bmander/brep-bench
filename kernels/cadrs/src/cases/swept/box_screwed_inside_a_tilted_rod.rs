//! `swept/box-screwed-inside-a-rod` turned rigidly so that the rod's axis runs along
//! w = (1, 2, 2)/3, with u = (2, −2, 1)/3 and v = (2, 1, −2)/3: a rod (radius 5, from the origin
//! to 20w) with a sealed helical tunnel, everything the unit cube 2.5 ≤ p·u ≤ 3.5,
//! −0.5 ≤ p·v ≤ 0.5, 2 ≤ p·w ≤ 3 passes through as it is screwed about the rod's axis, rising 4
//! a turn, from 0° to 720°.
//!
//! The cube is a square sketched on the plane through 2.5u − 0.5v + 2w square to w (its x axis
//! along u), extruded 1 along w; its screw is cadrs's sweep of a solid, cut from the rod.
use crate::bench::{cut, cylinder, extrude, p3, v3};
use crate::shared::swept::screwed;
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let (u, w) = (v3(2.0, -2.0, 1.0) / 3.0, v3(1.0, 2.0, 2.0) / 3.0);
    let o = p3(0.0, 0.0, 0.0);
    let face = Profile::of(Plane::new(p3(2.0, -0.5, 2.5), u, w), vec![Loop::rectangle(0.0, 0.0, 1.0, 1.0)]);
    let tool = extrude(k, &face, 1.0)?;
    let swept = screwed(k, tool, Axis::new(o, w), 4.0, 0.0, 720.0)?;
    let r = cylinder(k, o, w, 5.0, 20.0)?;
    cut(k, r, swept)
}
