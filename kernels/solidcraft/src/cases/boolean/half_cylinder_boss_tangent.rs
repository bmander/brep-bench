use crate::solids::*;

pub fn build() -> R<Body> {
    let cube = block((0.0, 0.0, 0.0), (10.0, 10.0, 10.0))?;
    union(&cube, &zcyl(10.0, 5.0, 5.0, 0.0, 10.0)?)
}
