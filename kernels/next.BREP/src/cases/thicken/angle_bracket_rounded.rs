//! The sheet of two faces meeting at a right angle along a crease, the floor from (0, 0, 0) to
//! (10, 8, 0) and the wall from (0, 0, 0) to (0, 8, 10), thickened by 1 to the outside of the
//! crease (−z and −x), the gap the moved faces leave closed by the quarter of the cylinder of
//! radius 1 about the crease.
//!
//! next.BREP thickens one face at a time, so a sheet of two faces is thickened as two faces of a
//! solid, the box they bound, by the kernel's shell grown outward (the box's four other faces its
//! openings), which extends the moved faces until they meet. That corner is then rounded to
//! radius 1, which puts the round on the cylinder of radius 1 about the crease, 1 from both moved
//! faces.
use crate::solids::offset::{bracket, shell};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (block, openings) = bracket()?;
    let angle = shell(&block, &openings, -1.0)?;
    Ok(fillet_edges(&angle, &[p(-1.0, 4.0, -1.0)], None, 1.0, false, None)?)
}
