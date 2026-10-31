use crate::solids::*;

pub fn build() -> R<Body> {
    let slab = block((0.0, 0.0, 0.0), (10.0, 4.0, 6.0))?;
    solidcraft_kernel::chamfer(&slab, &[v3(5.0, 0.0, 6.0), v3(5.0, 4.0, 6.0)], 2.0)
}
