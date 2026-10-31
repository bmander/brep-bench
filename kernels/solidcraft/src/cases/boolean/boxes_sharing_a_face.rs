use crate::solids::*;

pub fn build() -> R<Body> {
    let a = block((0.0, 0.0, 0.0), (10.0, 10.0, 10.0))?;
    let b = block((10.0, 0.0, 0.0), (20.0, 10.0, 10.0))?;
    union(&a, &b)
}
