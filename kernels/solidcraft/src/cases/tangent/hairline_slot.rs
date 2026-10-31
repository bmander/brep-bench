use crate::solids::*;

pub fn build() -> R<Body> {
    let cube = block((0.0, 0.0, 0.0), (10.0, 10.0, 10.0))?;
    cut(&cube, &block((5.0, -1.0, 5.0), (5.000001, 11.0, 11.0))?)
}
