use crate::solids::*;

pub fn build() -> R<Body> {
    let cube = block((0.0, 0.0, 0.0), (10.0, 10.0, 10.0))?;
    let mut edges = Vec::new();
    for a in [0.0, 10.0] {
        for b in [0.0, 10.0] {
            edges.extend([v3(5.0, a, b), v3(a, 5.0, b), v3(a, b, 5.0)]);
        }
    }
    solidcraft_kernel::chamfer(&cube, &edges, 2.0)
}
