//! An L-block, the polygon through (x, z) = (0, 0), (10, 0), (10, 3), (3, 3), (3, 10), (0, 10)
//! extruded from y = 0 to y = 10, its concave inside edge (x = 3, z = 3, along y) chamfered with
//! the same distance 7 on both faces, adding material: the chamfer runs from (10, y, 3) to
//! (3, y, 10), consuming both faces at the edge, and the section becomes the pentagon through
//! (0, 0), (10, 0), (10, 3), (3, 10), (0, 10).
//!
//! The L is sketched on the Front plane and extruded along +y; the inside edge is picked by a
//! point on it and chamfered, equal distance 7.
use crate::bench::{extrude, p3};
use crate::shared::chamfer::chamfer;
use crate::Refusal;
use cadrs_kernel::{BodyId, ChamferSpec, DefaultKernel, Loop, Pick, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let l = Loop::polygon(&[[0.0, 0.0], [10.0, 0.0], [10.0, 3.0], [3.0, 3.0], [3.0, 10.0], [0.0, 10.0]]);
    // the Front plane's normal is −y: extrude against it, along +y
    let block = extrude(k, &Profile::of(Plane::xz(0.0), vec![l]), -10.0)?;
    let inside = k.edge_at(block, p3(3.0, 5.0, 3.0))?;
    chamfer(k, block, &[inside], ChamferSpec::EqualDistance(7.0))
}
