use crate::solids::*;

/// The middle of each of the cube's twelve edges. What is left of an edge keeps its middle, each
/// chamfer before it cutting as much off either end, so it picks the edge on the solid at hand.
fn middles() -> Vec<Vec3> {
    let mut edges = Vec::new();
    for a in [0.0, 10.0] {
        for b in [0.0, 10.0] {
            edges.extend([v3(5.0, a, b), v3(a, 5.0, b), v3(a, b, 5.0)]);
        }
    }
    edges
}

pub fn build() -> R<Body> {
    // Each edge its own chamfer, ending on the faces at its ends, the earlier chamfers among
    // them, so the three at a corner mitre.
    let mut solid = block((0.0, 0.0, 0.0), (10.0, 10.0, 10.0))?;
    for edge in middles() {
        solid = solidcraft_kernel::chamfer(&solid, &[edge], 2.0)?;
    }
    Ok(solid)
}
