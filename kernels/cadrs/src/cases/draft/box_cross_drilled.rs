//! The box from the origin to (30, 20, 20), drilled right through along x by a hole of radius 4
//! about the line y = 10, z = 10. Then its faces x = 0 and x = 30, where the hole comes out, are
//! drafted 8°, pull +z, about the neutral plane z = 0: each leans in as it rises, the hole's wall
//! now trimmed by them in ellipses.
//!
//! The rectangle extruded; the hole a circle sketched on a plane square to x, extruded through
//! the box and cut; then cadrs's Draft of the two end faces.
use crate::bench::{block, cut, cylinder, p3, v3};
use crate::shared::draft::drafted;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = block(k, p3(0.0, 0.0, 0.0), p3(30.0, 20.0, 20.0))?;
    let drill = cylinder(k, p3(-1.0, 10.0, 10.0), v3(1.0, 0.0, 0.0), 4.0, 32.0)?;
    let drilled = cut(k, b, drill)?;
    let ends = k.faces_at(drilled, &[p3(0.0, 2.0, 2.0), p3(30.0, 2.0, 2.0)])?;
    drafted(k, drilled, ends, 8.0, v3(0.0, 0.0, 1.0), p3(0.0, 0.0, 0.0))
}
