//! A block (40 by 40 by 8) with an L-shaped pocket cut 5 into its top (the polygon through
//! (−12, −12), (12, −12), (12, 0), (0, 0), (0, 12) and (−12, 12), its five convex corners rounded
//! to radius 1 and the re-entrant one at the origin to radius 3), the rim of the pocket's floor
//! rounded to radius 1.
//!
//! The block is a rectangle extruded; the pocket its rounded outline on the plane z = 3 extruded
//! 5, cut away; then cadrs's Fillet on the twelve edges of the floor's rim.
use crate::bench::{block, cut, extrude, p3};
use crate::shared::fillet::{edges_at_height, fillet, rounded_polygon};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = block(k, p3(-20.0, -20.0, 0.0), p3(20.0, 20.0, 8.0))?;
    let outline = rounded_polygon(
        &[(-12.0, -12.0), (12.0, -12.0), (12.0, 0.0), (0.0, 0.0), (0.0, 12.0), (-12.0, 12.0)],
        &[1.0, 1.0, 1.0, 3.0, 1.0, 1.0],
    );
    let pocket = extrude(k, &Profile::of(Plane::xy(3.0), vec![Loop::new(outline)]), 5.0)?;
    let cut = cut(k, b, pocket)?;
    let rim = edges_at_height(k, cut, 3.0)?;
    fillet(k, cut, &rim, 1.0)
}
