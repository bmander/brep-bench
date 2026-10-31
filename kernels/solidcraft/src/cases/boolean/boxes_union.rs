use crate::solids::*;

pub fn build() -> R<Body> {
    let a = block((0.0, 0.0, 0.0), (10.0, 10.0, 10.0))?;
    let b = block((5.0, 5.0, 0.0), (15.0, 15.0, 10.0))?;
    union(&a, &b)
}
