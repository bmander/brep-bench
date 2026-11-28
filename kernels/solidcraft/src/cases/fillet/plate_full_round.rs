use crate::solids::*;

/// Every edge of the plate rounded to half its thickness in one call, each picked at its middle.
pub fn build() -> R<Body> {
    let plate = block((0.0, 0.0, 0.0), (40.0, 30.0, 6.0))?;
    let mut edges = Vec::new();
    for z in [0.0, 6.0] {
        edges.extend([v3(20.0, 0.0, z), v3(20.0, 30.0, z), v3(0.0, 15.0, z), v3(40.0, 15.0, z)]);
    }
    for (x, y) in [(0.0, 0.0), (40.0, 0.0), (40.0, 30.0), (0.0, 30.0)] {
        edges.push(v3(x, y, 3.0));
    }
    solidcraft_kernel::fillet(&plate, &edges, 3.0)
}
