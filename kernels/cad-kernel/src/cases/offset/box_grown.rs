//! The box from the origin to (10, 8, 6), grown by 1: every point within 1 of it.
//!
//! Refused. cad-kernel has no offset of a solid: neither growing nor shrinking one is in its API
//! (its README lists "shelling and offsets" among what comes next).
//!
//! Solvent's `std.rounds.Grown` builds this one exactly, but geomsolver's lowering
//! (`solid::grown`) builds it, not cad-kernel. The lowering makes a piece for each face (its slab),
//! each edge (a wedge of a cylinder) and each corner (a sector of a ball), and cad-kernel only
//! glues the pieces to the body (a recipe's `grown` node). Building those pieces here would be the
//! caller making the offset, not the kernel.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("no offset of a solid: cad-kernel cannot grow a body (Solvent's std.rounds.Grown builds this one, but geomsolver's lowering places each face's slab, edge's wedge and corner's ball sector, and cad-kernel only glues them)".into())
}
