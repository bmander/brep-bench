//! A bar (the box from (−40, −8, −5) to (40, 8, 5)) joined to an eye across its middle (the
//! cylinder of radius 14 about the y axis from y = −8 to 8), the four concave edges where the
//! eye's side meets the bar's top and bottom rounded to radius 3.
//!
//! The bar is a rectangle extruded and the eye a circle extruded, united; then cadrs's Fillet
//! on the four straight edges along y at x = ±√171, z = ±5.
use crate::bench::{block, cylinder, p3, union, v3};
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let bar = block(k, p3(-40.0, -8.0, -5.0), p3(40.0, 8.0, 5.0))?;
    let eye = cylinder(k, p3(0.0, -8.0, 0.0), v3(0.0, 1.0, 0.0), 14.0, 16.0)?;
    let joined = union(k, bar, eye)?;
    let x = 171f64.sqrt();
    let on = |p: &crate::bench::P3| (p.x.abs() - x).abs() < 1e-6 && (p.z.abs() - 5.0).abs() < 1e-6;
    let edges = k.edges_where(joined, &|e| on(&e.start) && on(&e.end) && on(&e.mid))?;
    if edges.len() != 4 {
        return Err(Refusal::from(format!("{} edges where the eye meets the bar, not 4", edges.len())));
    }
    fillet(k, joined, &edges, 3.0)
}
