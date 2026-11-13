//! An L-shaped block shelled outward open at its step, the walls beyond the opening kept. The
//! outline through (0, 0), (20, 0), (20, 20), (15, 20), (15, 5) and (0, 5), extruded 20 along +z,
//! shelled outward to a wall 1 thick, open at the face y = 5, its edges sharp.
use brepkit_operations::offset_v2::shell_v2;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::hollow::{face_through, plate};
use crate::solids::p;
use crate::Refusal;

const OUTLINE: [(f64, f64); 6] = [(0.0, 0.0), (20.0, 0.0), (20.0, 20.0), (15.0, 20.0), (15.0, 5.0), (0.0, 5.0)];

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let l = plate(topo, &OUTLINE, 20.0)?;
    let step = face_through(topo, l, p(7.5, 5.0, 10.0))?;
    Ok(shell_v2(topo, l, 1.0, &[step])?)
}
