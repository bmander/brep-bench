//! An L-shaped block: the outline through (0, 0), (30, 0), (30, 10), (10, 10), (10, 25) and
//! (0, 25) extruded from z = 0 to z = 12. All six sides drafted 6°, pull +z, about the neutral
//! plane z = 0, each leaning into the material as it rises; the two at the concave edge meet
//! along a new edge where their planes cross.
//!
//! The outline extruded, then cadrs's Draft of every side.
use crate::bench::{extrude, p3, v3};
use crate::shared::draft::{drafted, is_side};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Pick, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let outline =
        Loop::polygon(&[[0.0, 0.0], [30.0, 0.0], [30.0, 10.0], [10.0, 10.0], [10.0, 25.0], [0.0, 25.0]]);
    let l = extrude(k, &Profile::of(Plane::xy(0.0), vec![outline]), 12.0)?;
    let sides = k.faces_where(l, &is_side)?;
    drafted(k, l, sides, 6.0, v3(0.0, 0.0, 1.0), p3(0.0, 0.0, 0.0))
}
