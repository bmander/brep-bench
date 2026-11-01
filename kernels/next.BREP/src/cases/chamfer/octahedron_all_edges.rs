//! The octahedron |x| + |y| + |z| ≤ 10, all twelve of its edges chamfered with distance √6 on both
//! faces in one chamfer, each corner where four meet cut flat by a square: the kernel's chamfer of
//! a selection stops the four chamfers at a corner where the corner's ball touches them and closes
//! the corner with the facet their end chords bound, at (10, 0, 0) the square in the plane x = 6.
use crate::solids::fillet::{octahedron, octahedron_edge_middles};
use crate::Refusal;
use brep_kernel::{fillet_edges, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let solid = octahedron(10.0)?;
    // The kernel's symmetric chamfer is the chord of the rolling ball's arc: its distance is the
    // ball's radius r, which touches each face r / tan(θ/2) from the edge, θ the faces' dihedral
    // angle, arccos(−1/3) for the octahedron. A setback of √6 is the ball of radius √6·tan(θ/2).
    let dihedral = (-1.0f64 / 3.0).acos();
    let radius = 6f64.sqrt() * (dihedral / 2.0).tan();
    Ok(fillet_edges(&solid, &octahedron_edge_middles(), None, radius, true, None)?)
}

