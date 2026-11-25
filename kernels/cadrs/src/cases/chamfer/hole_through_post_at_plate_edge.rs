//! A square plate (from (−50, −50, 0) to (50, 50, 20)) united with a post (radius 15 about the
//! upright through (45, 0), from z = 0 to 40) standing across its edge, drilled through the post
//! with a hole of radius 6, both rims of the hole chamfered 1.5 at 45°.
//!
//! The plate is a rectangle extruded, the post and the hole circles extruded; the post united
//! and the hole cut; then cadrs's Chamfer on the hole's two rims (the circles of radius 6 on the
//! bottom face z = 0 and on the post's top z = 40).
use crate::bench::{block, cut, cylinder, p3, union, v3, P3};
use crate::shared::chamfer::chamfer;
use crate::Refusal;
use cadrs_kernel::{BodyId, ChamferSpec, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let up = v3(0.0, 0.0, 1.0);
    let plate = block(k, p3(-50.0, -50.0, 0.0), p3(50.0, 50.0, 20.0))?;
    let post = cylinder(k, p3(45.0, 0.0, 0.0), up, 15.0, 40.0)?;
    let hole = cylinder(k, p3(45.0, 0.0, -1.0), up, 6.0, 42.0)?;
    let joined = union(k, plate, post)?;
    let drilled = cut(k, joined, hole)?;
    let rim = |p: &P3| ((p.x - 45.0).hypot(p.y) - 6.0).abs() < 1e-6 && (p.z.abs() < 1e-6 || (p.z - 40.0).abs() < 1e-6);
    let edges = k.edges_where(drilled, &|e| rim(&e.start) && rim(&e.mid) && rim(&e.end))?;
    if edges.is_empty() {
        return Err(Refusal::from("no rim of the hole"));
    }
    chamfer(k, drilled, &edges, ChamferSpec::EqualDistance(1.5))
}
