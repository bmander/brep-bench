//! A plate (from (−40, −25, 0) to (40, 25, 13), its upright edges rounded to radius 10) with a
//! lug on its north side (the cylinder of radius 9 about the upright through (0, 25)), the lug's
//! two concave edges with the side rounded to radius 8, then the bottom rim of the whole outline
//! rounded to radius 2.
//!
//! The plate is its rounded rectangle extruded and the lug a circle extruded, united; then
//! cadrs's Fillet on the two upright edges at (±9, 25), and another on every edge of the bottom.
use crate::bench::{cylinder, extrude, p3, union, v3, P3};
use crate::shared::fillet::{edges_at_height, fillet, rounded_polygon};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Pick, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let outline = rounded_polygon(&[(-40.0, -25.0), (40.0, -25.0), (40.0, 25.0), (-40.0, 25.0)], &[10.0; 4]);
    let plate = extrude(k, &Profile::of(Plane::xy(0.0), vec![Loop::new(outline)]), 13.0)?;
    let lug = cylinder(k, p3(0.0, 25.0, 0.0), v3(0.0, 0.0, 1.0), 9.0, 13.0)?;
    let joined = union(k, plate, lug)?;
    let at = |p: &P3| (p.x.abs() - 9.0).abs() < 1e-6 && (p.y - 25.0).abs() < 1e-6;
    let roots = k.edges_where(joined, &|e| at(&e.start) && at(&e.end) && at(&e.mid))?;
    if roots.len() != 2 {
        return Err(Refusal::from(format!("{} edges where the lug meets the side, not 2", roots.len())));
    }
    let blended = fillet(k, joined, &roots, 8.0)?;
    let rim = edges_at_height(k, blended, 0.0)?;
    fillet(k, blended, &rim, 2.0)
}
