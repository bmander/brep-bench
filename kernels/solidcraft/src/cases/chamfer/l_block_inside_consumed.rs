use crate::solids::*;

pub fn build() -> R<Body> {
    let l = polygon(&[(0.0, 0.0), (10.0, 0.0), (10.0, 3.0), (3.0, 3.0), (3.0, 10.0), (0.0, 10.0)]);
    let block = extrude(&xz(), l, -10.0, 0.0)?;
    solidcraft_kernel::chamfer(&block, &[v3(3.0, 5.0, 3.0)], 7.0)
}
