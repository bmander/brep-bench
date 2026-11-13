//! An L-shaped block shelled inward open at its step, the wall rising past the opening kept. The
//! outline through (0, 0), (20, 0), (20, 20), (15, 20), (15, 5) and (0, 5), extruded 20 along +z,
//! shelled inward to a wall 1 thick, open at the face y = 5, the corners sharp.
use brepkit_operations::shell_op::shell;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::hollow::{face_through, plate};
use crate::solids::p;
use crate::Refusal;

const OUTLINE: [(f64, f64); 6] = [(0.0, 0.0), (20.0, 0.0), (20.0, 20.0), (15.0, 20.0), (15.0, 5.0), (0.0, 5.0)];

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let l = plate(topo, &OUTLINE, 20.0)?;
    let step = face_through(topo, l, p(7.5, 5.0, 10.0))?;
    Ok(shell(topo, l, 1.0, &[step])?)
}
