//! The octahedron |x| + |y| + |z| ≤ 10, all twelve of its edges chamfered with distance √6 on both
//! faces, one after another, each edge its own chamfer ending on the faces at its ends (the
//! chamfers made before it among them), so the four at each corner mitre in a point, at
//! (10, 0, 0) the point (8, 0, 0).
use crate::solids::fillet::{octahedron, octahedron_edge_middles};
use crate::Refusal;
use brep_kernel::{fillet_edges, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let mut solid = octahedron(10.0)?;
    // The kernel's symmetric chamfer is the chord of the rolling ball's arc: its distance is the
    // ball's radius r, which touches each face r / tan(θ/2) from the edge, θ the faces' dihedral
    // angle, arccos(−1/3) for the octahedron. A setback of √6 is the ball of radius √6·tan(θ/2).
    let dihedral = (-1.0f64 / 3.0).acos();
    let radius = 6f64.sqrt() * (dihedral / 2.0).tan();
    // Each edge is selected by its middle, which the chamfers before it leave on what is left of
    // it.
    for middle in octahedron_edge_middles() {
        solid = fillet_edges(&solid, &[middle], None, radius, true, None)?;
    }
    Ok(solid)
}
