//! The block under the part 11/20 ≤ u, v ≤ 1 of the bicubic Bézier patch with control points
//! (xᵢ, xⱼ, 1/2 + sᵢsⱼ), the graph of z = 1/2 + ∛(xy) pinched to a point at u = v = 1/2, cut by
//! the plane −x + y + z/10 = 13/250, which passes 0.002 from that point.
//!
//! As the case allows, the top face is built on the patch cut down to that part (the same face),
//! by the geometry crate's de Casteljau subdivision.
use super::cuspidal_patch_sliced_through_its_cusp::{block_under_patch, piece};
use crate::Refusal;
use crate::solids::*;

pub fn build() -> Result<Body, Refusal> {
    let (xs, ss) = ([-1.0, 1.0, -1.0, 1.0], [-1.0, -1.0 / 3.0, 1.0 / 3.0, 1.0]);
    let net: Vec<Vec<Vec3>> =
        (0..4).map(|i| (0..4).map(|j| v3(xs[i], xs[j], 0.5 + ss[i] * ss[j])).collect()).collect();
    let block = block_under_patch(&piece(&net, 0.55, 0.55)?)?;
    // the plane through (0, 13/250, 0) whose normal x × y is (−1, 1, 1/10), toward the part cut
    // away: x along (1, 1, 0), level, and y = n × x, (−1/10, 1/10, −2), both made unit
    let x = v3(1.0, 1.0, 0.0) * (1.0 / 2f64.sqrt());
    let y = v3(-0.1, 0.1, -2.0) * (1.0 / 4.02f64.sqrt());
    let plane = Plane { origin: v3(0.0, 13.0 / 250.0, 0.0), x, y };
    let (below, _) = solidcraft_kernel::half_space(&block, &plane, false)?;
    Ok(below.ok_or("nothing of the block lies on the plane's near side")?)
}
